use coco_accelerated::{Context, Function, InputMatrix, IntoEnumIterator, Problem};
use criterion::{criterion_main, BenchmarkId, Criterion, Throughput};
use rand::{distributions, prelude::*};

const RAND_SEED: u64 = 0xdeadbeef;
const DIMENSIONS: [usize; 2] = [2, 40];
const BATCH_SIZES: [usize; 5] = [8, 16, 32, 64, 128];

pub fn functions_to_bench() -> Vec<Function> {
    let only = &[];
    let skip = &[];

    if only.is_empty() {
        Function::iter().filter(|f| !skip.contains(f)).collect()
    } else {
        only.to_vec()
    }
}

pub fn compare_function(c: &mut Criterion, function: Function) {
    #[cfg(feature = "reference")]
    let coco = &mut coco_accelerated::reference::Suite::new();

    let context = &mut Context::new();
    let generator = &mut rand::rngs::StdRng::seed_from_u64(RAND_SEED);

    let mut group = c.benchmark_group(function.to_string());
    for dimension in DIMENSIONS {
        let problem = &mut Problem::new(context, function, dimension);

        #[cfg(feature = "reference")]
        let reference = &mut problem.get_reference_instance(coco);

        for batch_size in BATCH_SIZES {
            let id = |name| BenchmarkId::new(name, format!("{dimension}x{batch_size}"));

            group.throughput(Throughput::Elements(batch_size as u64));
            let data = generator
                .sample_iter(distributions::Uniform::new_inclusive(-5.0, 5.0))
                .take(dimension * batch_size)
                .collect::<Vec<f64>>();

            let input = InputMatrix::new(&data, dimension);

            #[cfg(feature = "c")]
            group.bench_with_input(id("sequential"), &batch_size, |b, _dim| {
                b.iter(|| problem.eval_futhark_c(input))
            });

            #[cfg(feature = "multicore")]
            group.bench_with_input(id("multicore"), &batch_size, |b, _dim| {
                b.iter(|| problem.eval_futhark_multicore(input))
            });

            #[cfg(feature = "opencl")]
            group.bench_with_input(id("opencl"), &batch_size, |b, _dim| {
                b.iter(|| problem.eval_futhark_opencl(input))
            });

            #[cfg(feature = "cuda")]
            group.bench_with_input(id("cuda"), &batch_size, |b, _dim| {
                b.iter(|| problem.eval_futhark_cuda(input))
            });

            #[cfg(feature = "reference")]
            group.bench_with_input(id("reference"), &batch_size, |b, _dim| {
                b.iter(|| reference.eval_coco(input))
            });
        }
    }
}

pub fn batch_eval_benches() {
    let mut criterion = Criterion::default().configure_from_args();

    for function in functions_to_bench() {
        compare_function(&mut criterion, function);
    }
}

criterion_main!(batch_eval_benches);
