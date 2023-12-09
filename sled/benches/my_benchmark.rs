use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use sled::{Rgb, Sled};

fn setters(c: &mut Criterion) {
    let mut sled = Sled::new("./benches/config1.toml").unwrap();
    let white = Rgb::new(1.0, 1.0, 1.0);
    let center = sled.center_point();

    c.bench_function("set", |b| {
        b.iter(|| {
            sled.set(5, white).unwrap();
        });
    });

    c.bench_function("set_range", |b| {
        b.iter(|| {
            sled.set_range(5..205, white).unwrap();
        });
    });

    c.bench_function("for_each_in_range", |b| {
        b.iter(|| {
            sled.for_each_in_range(5..205, |led| led.color = white);
        });
    });

    c.bench_function("set_all", |b| {
        b.iter(|| {
            sled.set_all(white);
        });
    });

    c.bench_function("for_each", |b| {
        b.iter(|| {
            sled.for_each(|led| led.color = white);
        });
    });

    c.bench_function("set_segment", |b| {
        b.iter(|| {
            sled.set_segment(1, white).unwrap();
        });
    });

    c.bench_function("for_each_in_segment", |b| {
        b.iter(|| {
            sled.for_each_in_segment(1, |led, _| led.color = white)
                .unwrap();
        });
    });

    c.bench_function("set_vertex", |b| {
        b.iter(|| {
            sled.set_vertex(3, white).unwrap();
        });
    });

    c.bench_function("set_vertices", |b| {
        b.iter(|| {
            sled.set_vertices(white);
        });
    });

    c.bench_function("for_each_vertex", |b| {
        b.iter(|| {
            sled.for_each_vertex(|led| led.color = white);
        });
    });

    c.bench_function("set_at_angle", |b| {
        b.iter(|| {
            sled.set_at_angle(0.0, white).unwrap();
        })
    });

    c.bench_function("set_closest_to", |b| {
        b.iter(|| {
            sled.set_closest_to(center, white);
        })
    });

    c.bench_function("set_at_dist", |b| {
        b.iter(|| {
            sled.set_at_dist(2.5, white).unwrap();
        })
    });
    c.bench_function("set_at_dist_from", |b| {
        b.iter(|| {
            sled.set_at_dist_from(center, 2.5, white).unwrap();
        })
    });

    c.bench_function("set_within_dist", |b| {
        b.iter(|| {
            sled.set_within_dist(2.5, white).unwrap();
        })
    });

    c.bench_function("set_within_dist_from", |b| {
        b.iter(|| {
            sled.set_within_dist_from(center, 2.5, white).unwrap();
        })
    });

    c.bench_function("map_by_index", |b| {
        b.iter(|| {
            sled.map_by_index(|_| white);
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .significance_level(0.05)
        .sample_size(100)
        .measurement_time(Duration::from_secs_f32(6.0))
        .warm_up_time(Duration::from_secs_f32(3.0));
    targets = setters
}
criterion_main!(benches);
