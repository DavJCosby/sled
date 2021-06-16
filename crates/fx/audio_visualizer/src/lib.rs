mod pitch;

//extern crate anyhow;
extern crate clap;
extern crate cpal;
extern crate hound;

use slc::prelude::*;

use std::{
    f32::consts::PI,
    sync::mpsc::{sync_channel, Receiver, SyncSender},
    thread,
    time::{Duration, Instant},
};

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Stream,
};

const EXPECTED_SAMPLE_RATE: usize = 48000;
const SCAN_DURATION: f32 = 1.0 / 45.0;
const SAMPLE_CHUNK_SIZE: usize = 24;

const NUM_BUCKETS: usize = 75;
const SAMPLES_PER_BUCKET: usize =
    (EXPECTED_SAMPLE_RATE as f32 * SCAN_DURATION / SAMPLE_CHUNK_SIZE as f32) as usize + 1;

#[derive(Copy, Clone)]
struct Bucket {
    left_sum: f32,
    right_sum: f32,
    samples: [f32; SAMPLES_PER_BUCKET],
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
        };

        BucketContainer {
            buckets: [default_bucket; NUM_BUCKETS],
        }
    }
}

pub struct AudioVisualizer;

impl InputDevice for AudioVisualizer {
    fn start(&self, input_handle: RoomControllerInputHandle) {
        thread::spawn(move || {
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
                render_buckets(&bucket_container, &input_handle);

                for (_, _) in rx.try_iter() {
                    // flush leftover samples
                }
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

fn inv_lerp(a: f32, b: f32, c: f32) -> f32 {
    (c - a) / (b - a)
}

fn process_audio(
    rx: &Receiver<(f32, f32)>,
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

    for (left, right) in rx.try_iter() {
        if sample_counter + 1 >= SAMPLES_PER_BUCKET {
            break;
        }

        let l_abs = left.abs();
        let r_abs = right.abs();

        if previous_l > l_abs {
            previous_peak_l = (2.0 * previous_peak_l + previous_l) / 3.0;
        }

        if previous_r > r_abs {
            previous_peak_r = (2.0 * previous_peak_r + previous_r) / 3.0;
        }

        previous_l = l_abs;
        previous_r = r_abs;

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

            lower_bucket.left_sum += left * lower_occupancy;
            lower_bucket.right_sum += right * lower_occupancy;
            lower_bucket.samples[sample_counter] = left * lower_occupancy;
            lower_bucket.samples[sample_counter + 1] = right * lower_occupancy;
        }

        if alpha > 0.01 {
            let upper_bucket = &mut bucket_container.buckets[lower_bucket_index as usize];

            upper_bucket.left_sum += previous_peak_l * alpha;
            upper_bucket.right_sum += previous_peak_r * alpha;
            upper_bucket.samples[sample_counter] = left * alpha;
            upper_bucket.samples[sample_counter + 1] = right * alpha;
        }

        sample_counter += 2;
    }

    return (previous_peak_l, previous_peak_r);
}

fn render_buckets(bucket_container: &BucketContainer, input_handle: &RoomControllerInputHandle) {
    let mut write = input_handle.write().unwrap();
    write.set_all((0, 0, 0));

    let map = |angle| {
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

        let lower_bucket = bucket_container.buckets[target_lower as usize];
        let upper_bucket = bucket_container.buckets[target_upper as usize];

        let v_lower = lower_bucket.left_sum + lower_bucket.right_sum.powi(2);
        let v_lower = (v_lower / (v_lower + 35.0)) * 255.0;

        let v_upper = upper_bucket.left_sum + upper_bucket.right_sum.powi(2);
        let v_upper = (v_upper / (v_upper + 35.0)) * 255.0;

        let v = v_lower + (v_upper - v_lower) * bucket_alpha;

        (v as u8, v as u8, v as u8)
    };
    write.map_angle_to_color(&map);
}

fn get_stream(tx: SyncSender<(f32, f32)>) -> Stream {
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

fn write_input_data<T, U>(input: &[T], sender: &SyncSender<(f32, f32)>)
where
    T: cpal::Sample,
    U: cpal::Sample + hound::Sample,
{
    let mut iter = input.into_iter();
    loop {
        let mut left = 0.0;
        let mut right = 0.0;

        for _ in 0..SAMPLE_CHUNK_SIZE {
            let l = iter.next();
            if l.is_none() {
                return;
            }
            left += l.unwrap().to_f32();

            let r = iter.next();
            if r.is_none() {
                return;
            }

            right += r.unwrap().to_f32();
        }

        sender.send((left, right)).unwrap();
    }
}
