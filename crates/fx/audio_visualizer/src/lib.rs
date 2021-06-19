//mod pitch;

extern crate clap;
extern crate cpal;
extern crate hound;

use pitch_detection::detector::autocorrelation::AutocorrelationDetector;
use pitch_detection::detector::PitchDetector;
use pitch_detection::float::Float;

use slc::prelude::*;

use lab::Lab;

use std::{
    f32::consts::PI,
    sync::{
        mpsc::{sync_channel, Receiver, SyncSender},
        Arc, Mutex,
    },
    thread,
    time::{Duration, Instant},
};

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Stream,
};

const EXPECTED_SAMPLE_RATE: usize = 48000;
const SCAN_DURATION: f32 = 1.0 / 34.0;
const SAMPLE_CHUNK_SIZE: usize = 12;

const NUM_BUCKETS: usize = 69;
const SAMPLES_PER_BUCKET: usize = (EXPECTED_SAMPLE_RATE as f32 * SCAN_DURATION) as usize + 1;

#[derive(Copy, Clone)]
struct Bucket {
    left_sum: f32,
    right_sum: f32,
    samples: [f32; SAMPLES_PER_BUCKET],
    peaks: f32,
}

struct BucketContainer {
    buckets: [Bucket; NUM_BUCKETS],
}

impl BucketContainer {
    fn new() -> Self {
        let default_bucket = Bucket {
            left_sum: 0.0,
            right_sum: 0.0,
            samples: [0.0; SAMPLES_PER_BUCKET],
            peaks: 0.0,
        };

        BucketContainer {
            buckets: [default_bucket; NUM_BUCKETS],
        }
    }
}

pub struct AudioVisualizer;

#[derive(Copy, Clone)]
struct Spring {
    val: f32,
    vel: f32,
    targ: f32,
    k: f32,
    friction: f32,
}

impl Spring {
    pub fn new(val: f32, k: f32, friction: f32) -> Self {
        Spring {
            val,
            vel: 0.0,
            targ: val,
            k,
            friction,
        }
    }

    pub fn update(&mut self) {
        let d = self.targ - self.val;
        let f = d * self.k;
        self.vel = (self.vel * (1.0 - self.friction)) + f;
        self.val += self.vel;
    }
}

const REFRESH_TIMING: f32 = 1.0 / 240.0;

impl InputDevice for AudioVisualizer {
    fn start(&self, input_handle: RoomControllerInputHandle) {
        // collect data
        let default_spring_v = Spring::new(0.0, 0.04, 1.0);
        let default_spring_p = Spring::new(0.0, 0.03, 1.0);

        let vol_springs = [default_spring_v; NUM_BUCKETS];
        let thread_safe_vol = Arc::new(Mutex::new(vol_springs));
        let thread_safe2_vol = Arc::clone(&thread_safe_vol);

        let pitch_springs = [default_spring_p; NUM_BUCKETS];
        let thread_safe_pit = Arc::new(Mutex::new(pitch_springs));
        let thread_safe2_pit = Arc::clone(&thread_safe_pit);
        thread::spawn(move || {
            let mut detector =
                AutocorrelationDetector::<f32>::new(SAMPLES_PER_BUCKET, SAMPLES_PER_BUCKET);
            let (tx, rx) = sync_channel(1000000);

            let mut ll = 0.0;
            let mut lr = 0.0;
            let stream = get_stream(tx);
            loop {
                let mut bucket_container = BucketContainer::new();

                record_audio(&stream);
                let (last_l, last_r) = process_audio(&rx, ll, lr, &mut bucket_container);
                ll = last_l;
                lr = last_r;

                let mut write_v = thread_safe_vol.lock().unwrap();
                let mut write_p = thread_safe_pit.lock().unwrap();

                let mut c = 0;
                for bucket in bucket_container.buckets {
                    //let signal = vec![bucket_container.buckets];
                    //println!("{}", bucket.peaks);
                    if bucket.peaks > 4.0 {
                        let pitch =
                            detector.get_pitch(&bucket.samples, EXPECTED_SAMPLE_RATE, 0.1, 0.1);
                        match pitch {
                            Some(p) => {
                                //println!("{}", p.frequency);
                                (*write_p)[c].targ = p.frequency;
                            }
                            None => {}
                        };
                    }
                    (*write_v)[c].targ = bucket.left_sum + bucket.right_sum;

                    c += 1;
                }
            }
        });

        // smooth and display
        thread::spawn(move || {
            let start = Instant::now();
            let mut last = 0.0;
            loop {
                let duration = start.elapsed().as_secs_f32();
                if duration - last < REFRESH_TIMING {
                    continue;
                }

                let mut write_v = thread_safe2_vol.lock().unwrap();
                let bc1 = *write_v;
                let mut write_p = thread_safe2_pit.lock().unwrap();
                let bc2 = *write_p;

                for spring in write_v.iter_mut() {
                    spring.update();
                    //println!("{}, {}", spring.val, spring.targ);
                }
                for spring in write_p.iter_mut() {
                    spring.update();
                    //println!("{}, {}", spring.val, spring.targ);
                }

                render_buckets(&bc1, &bc2, &input_handle);
                last = duration;
            }
        });
    }

    fn stop(&mut self) {
        todo!()
    }
}

fn record_audio(stream: &Stream) {
    stream.play().unwrap();
    let start = Instant::now();

    let done_at = Duration::from_secs_f32(SCAN_DURATION);

    loop {
        if start.elapsed() >= done_at {
            break;
        }
    }
    stream.pause().unwrap();
}

fn process_audio(
    rx: &Receiver<(f32, f32, [f32; SAMPLE_CHUNK_SIZE * 2])>,
    ll: f32,
    lr: f32,
    bucket_container: &mut BucketContainer,
) -> (f32, f32) {
    let mut sample_counter = 0;

    let mut previous_peak_l = ll;
    let mut previous_peak_r = lr;

    let mut previous_l = 0.0;
    let mut previous_r = 0.0;

    //println!();

    let mut max_l = 0.0;
    let mut max_r = 0.0;

    for (left, right, samples) in rx.try_iter() {
        let l_abs = left.abs();
        let r_abs = right.abs();

        max_l = l_abs.max(max_l);
        max_r = r_abs.max(max_r);

        let mut unreported_peak = false;

        if previous_l > l_abs {
            previous_peak_l = (previous_peak_l * 2.0 + previous_l) / 3.0;
            unreported_peak = true;
        }

        if previous_r > r_abs {
            previous_peak_r = (previous_peak_r * 2.0 + previous_r) / 3.0;
            unreported_peak = true;
        }

        previous_l = l_abs;
        previous_r = r_abs;

        if sample_counter + 1 < SAMPLES_PER_BUCKET {
            let dir = previous_peak_r / (previous_peak_l + previous_peak_r);
            let target_exact = (dir * (NUM_BUCKETS + 1) as f32) - 1.0;
            let lower_bucket_index = target_exact.floor();
            let upper_bucket_index = target_exact.ceil();
            let alpha = target_exact - lower_bucket_index as f32;

            if dir > 1.0 {
                sample_counter += 2;
                continue;
            }

            if alpha < 0.99 && (upper_bucket_index < NUM_BUCKETS as f32) {
                let lower_bucket = &mut bucket_container.buckets[upper_bucket_index as usize];
                let lower_occupancy = 1.0 - alpha;

                lower_bucket.left_sum += left.abs() * lower_occupancy;
                lower_bucket.right_sum += right.abs() * lower_occupancy;

                let mut c = 0;
                for sample in samples {
                    lower_bucket.samples[sample_counter * SAMPLE_CHUNK_SIZE / 4 + c] =
                        sample * lower_occupancy;
                    c += 1;
                }

                if unreported_peak {
                    lower_bucket.peaks += 1.0;
                }
            }

            if alpha > 0.01 {
                let upper_bucket = &mut bucket_container.buckets[lower_bucket_index as usize];

                upper_bucket.left_sum += previous_peak_l * alpha;
                upper_bucket.right_sum += previous_peak_r * alpha;

                let mut c = 0;
                for sample in samples {
                    upper_bucket.samples[sample_counter * SAMPLE_CHUNK_SIZE / 4 + c] =
                        sample * alpha;
                    c += 1;
                }

                if unreported_peak {
                    upper_bucket.peaks += 1.0;
                }
            }
        }
        sample_counter += 2;
    }

    if max_l == 0.0 {
        previous_peak_l = 0.0;
    }

    if max_r == 0.0 {
        previous_peak_r = 0.0;
    }

    return (previous_peak_l, previous_peak_r);
}

fn lerp(a: (f32, f32, f32), b: (f32, f32, f32), t: f32) -> (f32, f32, f32) {
    (
        a.0 + (b.0 - a.0) * t,
        a.1 + (b.1 - a.1) * t,
        a.2 + (b.2 - a.2) * t,
    )
}

fn color_curve1(t: f32) -> (f32, f32, f32) {
    let p1 = (77.0, 61.0, 54.0);
    let p2 = (57.0, -17.0, 70.0);
    let p3 = (0.0, -23.0, 93.0);
    let p4 = (-23.0, -28.0, 93.0);
    let p5 = (-69.0, 31.0, 93.0);

    if t < 0.25 {
        return lerp(p1, p2, t * 4.0);
    } else if t >= 0.25 && t < 0.5 {
        return lerp(p2, p3, (t - 0.25) * 4.0);
    } else if t >= 0.5 && t < 0.75 {
        return lerp(p3, p4, (t - 0.5) * 4.0);
    } else {
        return lerp(p4, p5, (t - 0.75) * 4.0);
    }
}

fn render_buckets(
    vol: &[Spring; NUM_BUCKETS],
    pit: &[Spring; NUM_BUCKETS],
    input_handle: &RoomControllerInputHandle,
) {
    let mut write = input_handle.write().unwrap();
    write.set_all((0, 0, 0));

    let map = |angle: f32| {
        if angle > PI || angle < 0.0 {
            return (0, 0, 0);
        }
        let alpha: f32 = 1.0 - (angle / PI);

        let target_exact = alpha * NUM_BUCKETS as f32;
        let target_lower = target_exact.floor();
        let mut target_upper = target_exact.ceil();

        if target_upper as usize >= NUM_BUCKETS {
            target_upper = target_lower;
        }

        let bucket_alpha = target_exact - target_lower;
        let lower_bucket = vol[target_lower as usize];
        let upper_bucket = vol[target_upper as usize];

        let v_lower = lower_bucket.val;
        let v_lower = (v_lower / (v_lower + 25.0)) * 255.0;

        let v_upper = upper_bucket.val;
        let v_upper = (v_upper / (v_upper + 25.0)) * 255.0;

        let v = v_lower + (v_upper - v_lower) * bucket_alpha;

        let lower_bucket_p = pit[target_lower as usize];
        let upper_bucket_p = pit[target_upper as usize];
        let p_lower = lower_bucket_p.val;
        let p_upper = upper_bucket_p.val;
        let p = p_lower + (p_upper - p_lower) * bucket_alpha;

        let p_zero_one = (p / 1500.0).min(1.0);

        //let pit_alpha =
        let (a, b, l) = color_curve1(p_zero_one);

        let lab = Lab { l, a, b };

        let tone = lab.to_rgb_normalized();

        //let v = 255.0;
        let darkened = (tone[0] as f32 * v, tone[1] as f32 * v, tone[2] as f32 * v);
        //(255, 0, 0)
        //(v as u8, v as u8, v as u8)
        return (darkened.0 as u8, darkened.1 as u8, darkened.2 as u8);
    };
    write.map_angle_to_color(&map);
}

fn get_stream(tx: SyncSender<(f32, f32, [f32; SAMPLE_CHUNK_SIZE * 2])>) -> Stream {
    let opt = Opt::from_args();

    #[cfg(any(
        not(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd")),
        not(feature = "jack")
    ))]
    let host = cpal::default_host();

    let device = if opt.device == "default" {
        host.default_input_device()
    } else {
        host.input_devices()
            .unwrap()
            .find(|x| x.name().map(|y| y == opt.device).unwrap_or(false))
    }
    .expect("failed to find input device");

    println!("Input device: {}", device.name().unwrap());

    let config = device
        .default_input_config()
        .expect("Failed to get default input config");
    println!("Default input config: {:?}", config);

    let err_fn = move |err| {
        eprintln!("an error occurred on stream: {}", err);
    };
    let stream = device
        .build_input_stream(
            &config.into(),
            move |data, _: &_| {
                write_input_data::<f32, f32>(data, &tx);
            },
            err_fn,
        )
        .unwrap();

    return stream;
}

#[derive(Debug)]
struct Opt {
    #[cfg(all(
        any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd"),
        feature = "jack"
    ))]
    jack: bool,

    device: String,
}

impl Opt {
    fn from_args() -> Self {
        let app = clap::App::new("beep").arg_from_usage("[DEVICE] 'The audio device to use'");
        let matches = app.get_matches();
        let device = matches.value_of("DEVICE").unwrap_or("default").to_string();

        #[cfg(any(
            not(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd")),
            not(feature = "jack")
        ))]
        Opt { device }
    }
}

fn write_input_data<T, U>(
    input: &[T],
    sender: &SyncSender<(f32, f32, [f32; SAMPLE_CHUNK_SIZE * 2])>,
) where
    T: cpal::Sample,
    U: cpal::Sample + hound::Sample,
{
    let mut iter = input.into_iter();
    loop {
        let mut left = 0.0;
        let mut right = 0.0;

        let mut samples = [0.0; SAMPLE_CHUNK_SIZE * 2];

        let mut c = 0;
        for i in 0..SAMPLE_CHUNK_SIZE {
            let l = iter.next();
            if l.is_none() {
                return;
            }
            left += l.unwrap().to_f32();
            samples[c] = left;

            c += 1;

            let r = iter.next();
            if r.is_none() {
                return;
            }

            right += r.unwrap().to_f32();
            samples[c] = right;
        }

        sender.send((left, right, samples)).unwrap();
    }
}
