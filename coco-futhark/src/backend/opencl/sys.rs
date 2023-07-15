#![allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    improper_ctypes,
    deref_nullptr,
    dead_code,
    clippy::approx_constant,
    clippy::upper_case_acronyms
)]

include!(concat!(env!("OUT_DIR"), "/futhark/opencl/accelerated.rs"));
