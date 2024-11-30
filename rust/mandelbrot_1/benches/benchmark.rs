use criterion::{criterion_group, criterion_main, Criterion};
use mandelbrot_lib::calculate_mandelbrot;

fn benchmark_mandelbrot(c: &mut Criterion) {
    c.bench_function("mandelbrot", |b| b.iter(|| calculate_mandelbrot()));
}

criterion_group!(benches, benchmark_mandelbrot);
criterion_main!(benches);