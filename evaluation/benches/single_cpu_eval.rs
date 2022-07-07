use coco_accelerated::{Context, Function, Problem};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

pub fn compare_function(c: &mut Criterion, function: Function) {
    let context = &Context::new();
    let mut problem = Problem::new(context, function);

    let mut group = c.benchmark_group(function.name());
    for dim in [5, 10, 20, 40] {
        let input = black_box(vec![1.0; dim]);
        let id = |name| BenchmarkId::new(name, dim);

        group.bench_with_input(id("futhark_c"), &dim, |b, _dim| {
            b.iter(|| problem.eval_futhark_c(&input))
        });

        group.bench_with_input(id("futhark_multicore"), &dim, |b, _dim| {
            b.iter(|| problem.eval_futhark_multicore(&input))
        });

        group.bench_with_input(id("coco"), &dim, |b, _dim| {
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
