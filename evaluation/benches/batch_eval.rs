use coco_accelerated::{Context, Function, InputMatrix, IntoEnumIterator, Problem};
use criterion::{criterion_main, BenchmarkId, Criterion};
use rand::prelude::*;

const RAND_SEED: u64 = 0xdeadbeef;

pub fn functions_to_bench() -> Vec<Function> {
    let only = &[];
    let skip = &[];

    if only.is_empty() {
        Function::iter().filter(|f| !skip.contains(f)).collect()
    } else {
        only.to_vec()
    }
}

pub fn compare_function(c: &mut Criterion, function: Function, dimension: usize) {
    let context = &mut Context::new();
    let mut generator = rand::rngs::StdRng::seed_from_u64(RAND_SEED);
    let problem = &mut Problem::new(context, function, dimension);

    let mut group = c.benchmark_group(format!("{}-d{}", function.name(), dimension));
    for batch_size in [10, 25, 50, 100, 200] {
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

pub fn batch_eval_benches_d20() {
    let mut criterion = Criterion::default().configure_from_args();

    for function in functions_to_bench() {
        compare_function(&mut criterion, function, 20);
    }
}

criterion_main!(batch_eval_benches_d20);
