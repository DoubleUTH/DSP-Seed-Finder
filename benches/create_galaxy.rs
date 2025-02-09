use criterion::{criterion_group, criterion_main, Criterion};
use dsp_seed_finder::{create_galaxy, GameDesc};
use std::{cell::Cell, hint::black_box};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("create galaxy", |b| {
        b.iter(|| {
            for seed in 0..100 {
                let desc = GameDesc {
                    seed,
                    star_count: 64,
                    resource_multiplier: 1.0,
                    habitable_count: Cell::new(0),
                };
                black_box(create_galaxy(&desc));
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
