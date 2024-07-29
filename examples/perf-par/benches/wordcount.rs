use criterion::{criterion_group, criterion_main, Criterion};
use std::{fs::File, hint::black_box, io::Read};
use wc::counter;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("count_large", |b| {
        b.iter_with_setup(
            || Box::new(File::open("../fixtures/large.txt").unwrap()),
            |mut reader| {
                black_box(counter(black_box(&mut reader)).unwrap());
            },
        )
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
