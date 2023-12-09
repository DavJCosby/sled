use criterion::{criterion_group, criterion_main, Criterion};
use glam::Vec2;
use sled::{Rgb, Sled};

fn setters(c: &mut Criterion) {
    let mut sled = Sled::new("./benches/config1.toml").unwrap();
    let white = Rgb::new(1.0, 1.0, 1.0);
    let center = sled.center_point();

    c.bench_function("set()", |b| {
        b.iter(|| {
            sled.set(5, white).unwrap();
        });
    });

    c.bench_function("set_range()", |b| {
        b.iter(|| {
            sled.set_range(5..205, white).unwrap();
        });
    });

    c.bench_function("set_segment()", |b| {
        b.iter(|| {
            sled.set_segment(1, white).unwrap();
        });
    });

    c.bench_function("set_vertex()", |b| {
        b.iter(|| {
            sled.set_vertex(3, white).unwrap();
        });
    });

    c.bench_function("set_vertices()", |b| {
        b.iter(|| {
            sled.set_vertices(white);
        });
    });

    c.bench_function("set_at_angle()", |b| {
        b.iter(|| {
            sled.set_at_angle(0.0, white).unwrap();
        })
    });

    c.bench_function("set_closest_to()", |b| {
        b.iter(|| {
            sled.set_closest_to(center, white);
        })
    });

    c.bench_function("set_at_dist_from()", |b| {
        b.iter(|| {
            sled.set_at_dist_from(center, 2.5, white).unwrap();
        })
    });

    c.bench_function("set_within_dist_from()", |b| {
        b.iter(|| {
            sled.set_within_dist_from(center, 2.5, white).unwrap();
        })
    });

    c.bench_function("map_by_index()", |b| {
        b.iter(|| {
            sled.map_by_index(|_| white);
        })
    });

    c.bench_function("map_by_dist_from()", |b| {
        b.iter(|| {
            sled.map_by_dist_from(center, |_| white);
        })
    });
}

criterion_group! {
name = benches;
config = Criterion::default().significance_level(0.05).sample_size(500);
targets = setters
}
criterion_main!(benches);
