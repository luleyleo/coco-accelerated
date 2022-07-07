use coco_accelerated::{Context, Function, Problem};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::prelude::*;

const RAND_SEED: u64 = 0xdeadbeef;

pub fn compare_function(c: &mut Criterion, function: Function) {
    let context = &Context::new();
    let mut generator = rand::rngs::StdRng::seed_from_u64(RAND_SEED);
    let mut problem = Problem::new(context, function);

    let mut group = c.benchmark_group(function.name());
    for dim in [5, 10, 20, 40] {
        let id = |name| BenchmarkId::new(name, dim);

        let input = (0..dim)
            .into_iter()
            .map(|_| generator.gen_range(-5.0..=5.0))
            .collect::<Vec<f64>>();

        group.bench_with_input(id("futhark_c"), &dim, |b, _dim| {
            b.iter(|| problem.eval_futhark_c(&input))
        });

        group.bench_with_input(id("coco"), &dim, |b, _dim| {
            b.iter(|| problem.eval_coco(&input))
        });
    }
}

pub fn benchmark_sphere(c: &mut Criterion) {
    compare_function(c, Function::Sphere);
}

pub fn benchmark_ellipsoid(c: &mut Criterion) {
    compare_function(c, Function::Ellipsoid);
}

pub fn benchmark_schaffers1(c: &mut Criterion) {
    compare_function(c, Function::Schaffers1);
}

criterion_group!(
    benches,
    benchmark_sphere,
    benchmark_ellipsoid,
    benchmark_schaffers1
);
criterion_main!(benches);
