use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use std::{fs::File, hint::black_box};
use wc::counter;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("count_large", |b| {
        b.iter_batched(
            || File::open("../fixtures/large.txt").unwrap(),
            |mut reader| {
                black_box(counter(black_box(&mut reader)).unwrap());
            },
            BatchSize::LargeInput,
        )
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
