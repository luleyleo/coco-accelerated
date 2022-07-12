use coco_accelerated::{Context, Function, InputMatrix, Problem};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::prelude::*;

const RAND_SEED: u64 = 0xdeadbeef;

pub fn compare_function(c: &mut Criterion, function: Function) {
    let context = &Context::new();
    let mut generator = rand::rngs::StdRng::seed_from_u64(RAND_SEED);
    let problem = Problem::new(context, function);

    let mut group = c.benchmark_group(format!("{}-batch", function.name()));
    for batch_size in [2, 10, 50, 100] {
        let id = |name| BenchmarkId::new(name, batch_size);
        let dim = 20;

        let data = (0..(dim * batch_size))
            .into_iter()
            .map(|_| generator.gen_range(-5.0..=5.0))
            .collect::<Vec<f64>>();

        let input = InputMatrix::new(&data, dim);

        #[cfg(feature = "c")]
        group.bench_with_input(id("futhark_c"), &batch_size, |b, _dim| {
            b.iter(|| problem.eval_futhark_c(input))
        });

        #[cfg(feature = "multicore")]
        group.bench_with_input(id("futhark_multicore"), &batch_size, |b, _dim| {
            b.iter(|| problem.eval_futhark_multicore(input))
        });

        #[cfg(feature = "opencl")]
        group.bench_with_input(id("futhark_opencl"), &batch_size, |b, _dim| {
            b.iter(|| problem.eval_futhark_opencl(input))
        });

        #[cfg(feature = "cuda")]
        group.bench_with_input(id("futhark_cuda"), &batch_size, |b, _dim| {
            b.iter(|| problem.eval_futhark_cuda(input))
        });

        #[cfg(feature = "reference")]
        group.bench_with_input(id("coco"), &batch_size, |b, _dim| {
            b.iter(|| problem.eval_coco(input))
        });
    }
}

pub fn benchmark_sphere(c: &mut Criterion) {
    compare_function(c, Function::Sphere);
}

pub fn benchmark_schaffers1(c: &mut Criterion) {
    compare_function(c, Function::Schaffers1);
}

criterion_group!(batch_eval_benches, benchmark_sphere, benchmark_schaffers1);
criterion_main!(batch_eval_benches);
