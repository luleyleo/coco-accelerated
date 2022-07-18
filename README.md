# Coco Accelerated
A hardware accelerated implementation of the [Coco BBOB suite](https://github.com/numbbo/coco).

## Requirements

For everything to work properly you need
- Rust
- Criterion (`cargo install cargo-criterion`)
- GCC or Clang
- Cuda SDK
- OpenCL SDK

Cuda and OpenCL can be avoided by always changing to the right directories and not using the respective features, but it is easier when everything is installed.

## Running Tests and Benchmarks

Run all tests:
```sh
$ cargo test
```

Run the batch benchmark:
```sh
$ cargo criterion --bench batch_eval --no-default-features --features [reference c multicore cuda]
```
The output can be found under `/reports/current/reports/index.html`.

## Project structure

* [coco-accelerated/](./coco-accelerated/coco-accelerated) - Rust wrapper around the Futhark implementation.
* [coco-futhark/](./coco-accelerated/coco-futhark) - Raw futhark implementation.
  * [c/](./coco-accelerated/coco-futhark/c) - Buildfiles for the respective backend.
  * [cuda/](./coco-accelerated/coco-futhark/cuda)
  * [futharkc/](./coco-accelerated/coco-futhark/futharkc)
  * [multicore/](./coco-accelerated/coco-futhark/multicore)
  * [opencl/](./coco-accelerated/coco-futhark/opencl)
  * [shared/](./coco-accelerated/coco-futhark/shared) - Contains the actual implementation.
* [coco-legacy/](./coco-accelerated/coco-legacy) - Bindings to the Coco code for generating random numbers.
* [evaluation/](./coco-accelerated/evaluation)
  * [benches/](./coco-accelerated/evaluation/benches) The benchmarks.
  * [src/](./coco-accelerated/evaluation/src) Just some dummy library.
  * [tests/](./coco-accelerated/evaluation/tests) Test suite to checking compliance with the reference implementation.
* [reports/](./coco-accelerated/reports) Directory where reports can be kept.
  * [current/](./coco-accelerated/reports/current) The current report.