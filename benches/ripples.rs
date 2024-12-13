mod drivers;
use drivers::ripples;

use spatial_led::Sled;
use std::time::Duration;

fn ripples(c: &mut Criterion) {
    let sled = Sled::new("./benches/config.yap").unwrap();
    let mut driver = ripples::build_driver();
    driver.mount(sled);

    let simulated_duration = 15.0;
    let simulated_hz = 60.0;
    let total_steps = (simulated_duration * simulated_hz) as usize;
    let timestep = Duration::from_secs_f32(1.0 / simulated_hz);
    let mut r = 0.0;
    c.bench_function("ripples", |b| {
        b.iter(|| {
            for _ in 0..total_steps {
                driver.step_by(timestep);

                let mut colors = driver.colors();
                r = colors.next().unwrap().red;
            }
        });
    });
    println!("{}", r); // prevent compiler from optimizing away output steps
}

use criterion::{criterion_group, criterion_main, Criterion};

criterion_group! {
    name = benches;
    config = Criterion::default()
        .significance_level(0.05)
        .sample_size(30)
        .warm_up_time(Duration::from_secs_f32(5.0))
        .measurement_time(Duration::from_secs_f32(20.0));
    targets = ripples
}
criterion_main!(benches);
