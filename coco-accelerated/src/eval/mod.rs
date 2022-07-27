mod futhark;

#[cfg(feature = "c")]
pub mod futhark_c;

#[cfg(feature = "multicore")]
pub mod futhark_multicore;

#[cfg(feature = "opencl")]
pub mod futhark_opencl;

#[cfg(feature = "cuda")]
pub mod futhark_cuda;
