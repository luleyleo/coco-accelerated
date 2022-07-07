use coco_accelerated::{Context, Function, Problem, DIMENSIONS};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

pub fn compare_function(c: &mut Criterion, function: Function) {
    let mut group = c.benchmark_group(function.name());
    for dim in DIMENSIONS {
        group.bench_with_input(BenchmarkId::new("accelerated", dim), dim, |b, dim| {
            let context = &Context::new();
            let mut problem = Problem::new(context, function);
            let input = vec![1.0; *dim];

            b.iter(|| problem.eval_accelerated(&input))
        });
        group.bench_with_input(BenchmarkId::new("coco", dim), dim, |b, dim| {
            let context = &Context::new();
            let mut problem = Problem::new(context, function);
            let input = vec![1.0; *dim];

            b.iter(|| problem.eval_coco(&input))
        });
    }
}

pub fn benchmark_sphere(c: &mut Criterion) {
    compare_function(c, Function::Sphere);
}

pub fn benchmark_schaffers1(c: &mut Criterion) {
    compare_function(c, Function::Schaffers1);
}

criterion_group!(benches, benchmark_sphere, benchmark_schaffers1);
criterion_main!(benches);
