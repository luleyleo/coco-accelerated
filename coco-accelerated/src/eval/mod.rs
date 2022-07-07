mod coco;
pub use self::coco::coco;

mod futhark;
use futhark::eval_futhark;

#[cfg(feature = "c")]
mod futhark_c;
#[cfg(feature = "c")]
pub use futhark_c::futhark_c;

#[cfg(feature = "multicore")]
mod futhark_multicore;
#[cfg(feature = "multicore")]
pub use futhark_multicore::futhark_multicore;
