use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use p3_field::{AbstractField, Field};
use p3_goldilocks::Goldilocks;

type F = Goldilocks;

fn try_inverse(c: &mut Criterion) {
    c.bench_function("try_inverse", |b| {
        b.iter_batched(
            rand::random::<F>,
            |x| x.try_inverse(),
            BatchSize::SmallInput,
        )
    });
}

fn square(c: &mut Criterion) {
    let x = rand::random::<F>();
    c.bench_function("try_square", |b| {
        b.iter(|| black_box(black_box(x).square()))
    });
}

fn add(c: &mut Criterion) {
    let x = rand::random::<F>();
    let y = rand::random::<F>();
    c.bench_function("try_add", |b| {
        b.iter(|| black_box(black_box(x) + black_box(y)))
    });
}

fn mul(c: &mut Criterion) {
    let x = rand::random::<F>();
    let y = rand::random::<F>();
    c.bench_function("try_mul", |b| {
        b.iter(|| black_box(black_box(x) * black_box(y)))
    });
}

criterion_group!(goldilocks_arithmetic, try_inverse, add, mul, square);
criterion_main!(goldilocks_arithmetic);
