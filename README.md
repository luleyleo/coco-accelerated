# Coco Accelerated
A hardware accelerated implementation of the [Coco BBOB suite](https://github.com/numbbo/coco).

## Requirements

For everything to work properly you need
- [Rust](https://www.rust-lang.org/)
- [Futhark](https://futhark-lang.org/)
- Criterion (`cargo install cargo-criterion`)
- GCC or Clang
- Cuda SDK (optional)
- OpenCL SDK (optional)

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

* [coco-accelerated/](./coco-accelerated/coco-accelerated) - Convenience wrapper around `coco-futhark`.
* [coco-futhark/](./coco-accelerated/coco-futhark) - Futhark implementation of COCO.
* [coco-legacy/](./coco-accelerated/coco-legacy) - Bindings to the Coco code for generating random numbers.
* [evaluation/](./coco-accelerated/evaluation)
  * [benches/](./coco-accelerated/evaluation/benches) The performance benchmarks.
  * [tests/](./coco-accelerated/evaluation/tests) Test suite to check compliance with the reference implementation.
  * [src/](./coco-accelerated/evaluation/src) Just some dummy library.
* [reports/](./coco-accelerated/reports) Directory where reports can be kept.
  * [current/](./coco-accelerated/reports/current) The current report.
  * [current/](./coco-accelerated/reports/current-both.zip) The evaluation I did for my bachelors thesis.