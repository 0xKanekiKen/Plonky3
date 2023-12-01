use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use p3_baby_bear::BabyBear;
use p3_field::{AbstractField, Field};

type F = BabyBear;

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

criterion_group!(baby_bear_arithmetic, try_inverse, square, add, mul);
criterion_main!(baby_bear_arithmetic);
