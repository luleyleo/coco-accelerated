use coco_accelerated::{Context, Function, Problem};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::prelude::*;

const RAND_SEED: u64 = 0xdeadbeef;

pub fn compare_function(c: &mut Criterion, function: Function) {
    let context = &Context::new();
    let mut generator = rand::rngs::StdRng::seed_from_u64(RAND_SEED);
    let problem = Problem::new(context, function);

    let mut group = c.benchmark_group(format!("{}-single", function.name()));
    for dim in [5, 10, 20, 40] {
        let id = |name| BenchmarkId::new(name, dim);

        let input = (0..dim)
            .into_iter()
            .map(|_| generator.gen_range(-5.0..=5.0))
            .collect::<Vec<f64>>();

        #[cfg(feature = "c")]
        group.bench_with_input(id("futhark_c"), &dim, |b, _dim| {
            b.iter(|| problem.eval_futhark_c_single(&input))
        });

        #[cfg(feature = "multicore")]
        group.bench_with_input(id("futhark_multicore"), &dim, |b, _dim| {
            b.iter(|| problem.eval_futhark_multicore_single(&input))
        });

        #[cfg(feature = "opencl")]
        group.bench_with_input(id("futhark_opencl"), &dim, |b, _dim| {
            b.iter(|| problem.eval_futhark_opencl_single(&input))
        });

        #[cfg(feature = "reference")]
        group.bench_with_input(id("coco"), &dim, |b, _dim| {
            b.iter(|| problem.eval_coco_single(&input))
        });
    }
}

pub fn benchmark_sphere(c: &mut Criterion) {
    compare_function(c, Function::Sphere);
}

pub fn benchmark_schaffers1(c: &mut Criterion) {
    compare_function(c, Function::Schaffers1);
}

criterion_group!(single_eval_benches, benchmark_sphere, benchmark_schaffers1);
criterion_main!(single_eval_benches);
