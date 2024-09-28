mod resources;
use resources::drivers::comet;

use spatial_led::Sled;
use std::time::Duration;

fn trail(c: &mut Criterion) {
    let sled = Sled::new("./benches/resources/config.yap").unwrap();
    let mut driver = comet::build_driver();
    driver.mount(sled);

    let simulated_duration = 30.0;
    let simulated_hz = 144.0;
    let total_steps = (simulated_duration * simulated_hz) as usize;
    let timestep = Duration::from_secs_f32(1.0 / simulated_hz);
    let mut r = 0;

    c.bench_function("comet", |b| {
        b.iter(|| {
            for _ in 0..total_steps {
                driver.step_by(timestep);

                let mut colors = driver.colors_coerced::<u8>();
                r = colors.next().unwrap().red;
            }
        });
    });
    println!("{}", r);
}

use criterion::{criterion_group, criterion_main, Criterion};

criterion_group! {
    name = benches;
    config = Criterion::default()
        .significance_level(0.05)
        .sample_size(50)
        .warm_up_time(Duration::from_secs_f32(10.0))
        .measurement_time(Duration::from_secs_f32(25.0));
    targets = trail
}
criterion_main!(benches);
