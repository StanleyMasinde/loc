use criterion::{Criterion, criterion_group, criterion_main};
use loc::cli::count_lines;
use std::{env, hint::black_box};

fn criterion_benchmark(c: &mut Criterion) {
    let path = env::current_dir().unwrap();
    c.bench_function("Count lines", |b| b.iter(|| count_lines(black_box(&path))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
