mod resources;
use resources::drivers::ripples;

use sled::Sled;
use std::time::Duration;

fn ripples(c: &mut Criterion) {
    let sled = Sled::new("./benches/resources/config.toml").unwrap();
    let mut driver = ripples::build_driver();
    driver.mount(sled);

    let simulated_duration = 30.0;
    let simulated_hz = 144.0;
    let total_steps = (simulated_duration * simulated_hz) as usize;
    let timestep = Duration::from_secs_f32(1.0 / simulated_hz);
    c.bench_function("ripples", |b| {
        b.iter(|| {
            for _ in 0..total_steps {
                driver.step_by(timestep);
                let colors = driver.colors::<u8>();
            }
        })
    });
}

use criterion::{criterion_group, criterion_main, Criterion};

criterion_group! {
    name = benches;
    config = Criterion::default()
        .significance_level(0.05)
        .sample_size(50)
        .warm_up_time(Duration::from_secs_f32(10.0))
        .measurement_time(Duration::from_secs_f32(25.0));
    targets = ripples
}
criterion_main!(benches);
