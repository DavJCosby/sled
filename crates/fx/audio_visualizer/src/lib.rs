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

const EXPECTED_SAMPLE_RATE: usize = 96_000;
const SCAN_DURATION: f32 = 1.0 / 24.0;

const NUM_BUCKETS: usize = 2;
const SAMPLES_PER_BUCKET: usize = (EXPECTED_SAMPLE_RATE as f32 * SCAN_DURATION) as usize + 1;

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
            let stream = get_stream(tx);

            loop {
                let mut bucket_container = BucketContainer::new();

                record_audio(&stream);
                process_audio(&rx, &mut bucket_container);
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

fn process_audio(rx: &Receiver<(f32, f32)>, bucket_container: &mut BucketContainer) {
    let mut sample_counter = 0;
    for (left, right) in rx.try_iter() {
        if sample_counter + 1 >= SAMPLES_PER_BUCKET {
            break;
        }
        let dir = right / (left + right);
        let target_exact = (dir * NUM_BUCKETS as f32) - 1.0;

        let lower_bucket_index = target_exact.floor();
        let upper_bucket_index = target_exact.ceil();

        let alpha = target_exact - lower_bucket_index as f32;
        // 0.0 = 100% lower, 1.0 = 100% upper

        if dir > 1.0 {
            sample_counter += 2;
            continue;
        }

        if alpha < 0.99 {
            let lower_bucket = &mut bucket_container.buckets[upper_bucket_index as usize];
            let lower_occupancy = 1.0 - alpha;

            lower_bucket.left_sum += left.abs() * lower_occupancy;
            lower_bucket.right_sum += right.abs() * lower_occupancy;
            lower_bucket.samples[sample_counter] = left * lower_occupancy;
            lower_bucket.samples[sample_counter + 1] = right * lower_occupancy;
        }

        if alpha > 0.01 {
            let upper_bucket = &mut bucket_container.buckets[lower_bucket_index as usize];

            upper_bucket.left_sum += left.abs() * alpha;
            upper_bucket.right_sum += right.abs() * alpha;
            upper_bucket.samples[sample_counter] = left * alpha;
            upper_bucket.samples[sample_counter + 1] = right * alpha;
        }

        sample_counter += 2;
    }
}

fn render_buckets(bucket_container: &BucketContainer, input_handle: &RoomControllerInputHandle) {
    let mut write = input_handle.write().unwrap();
    write.set_all((0, 0, 0));

    let mut bucket_counter = 0;
    for bucket in bucket_container.buckets.iter() {
        let (hz, amplitude) = pitch::detect(&bucket.samples);
        let vol = (amplitude * 255.0);
        if vol > 0.0 {
            let expected_pan = (NUM_BUCKETS - 1 - bucket_counter) as f32 / (NUM_BUCKETS - 1) as f32;
            let true_pan = bucket.left_sum / (bucket.left_sum + bucket.right_sum);
            let angle = (expected_pan + true_pan) * PI * 0.5;
            write.set_at_view_angle(angle, (vol as u8, vol as u8, vol as u8), true);
        }

        //println!("{}", vol);

        //total_left += bucket.left_sum;
        //total_right += bucket.right_sum;
        //println!("{}", bucket.right_sum + bucket.left_sum);
        bucket_counter += 1;
    }

    //println!("balance: {}", total_right / (total_left + total_right));
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
        let l = iter.next();
        if l.is_none() {
            return;
        }
        let left_sample = l.unwrap().to_f32();

        let r = iter.next();
        if r.is_none() {
            return;
        }

        let right_sample = r.unwrap().to_f32();

        sender.send((left_sample, right_sample)).unwrap();
    }
}
