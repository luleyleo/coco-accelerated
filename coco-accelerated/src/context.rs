use std::cell::RefCell;

pub struct Context {
    pub coco: RefCell<coco::Suite>,

    #[cfg(feature = "c")]
    pub futhark_c: coco_futhark_c::Context,

    #[cfg(feature = "multicore")]
    pub futhark_multicore: coco_futhark_multicore::Context,

    #[cfg(feature = "opencl")]
    pub futhark_opencl: coco_futhark_opencl::Context,

    #[cfg(feature = "cuda")]
    pub futhark_cuda: coco_futhark_cuda::Context,
}

impl Context {
    pub fn new() -> Self {
        let coco = RefCell::new(coco::Suite::new(coco::SuiteName::Bbob, "", "").unwrap());
        Context {
            coco,

            #[cfg(feature = "c")]
            futhark_c: coco_futhark_c::Context::default(),

            #[cfg(feature = "multicore")]
            futhark_multicore: coco_futhark_multicore::Context::default(),

            #[cfg(feature = "opencl")]
            futhark_opencl: coco_futhark_opencl::Context::default(),

            #[cfg(feature = "cuda")]
            futhark_cuda: coco_futhark_cuda::Context::default(),
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
