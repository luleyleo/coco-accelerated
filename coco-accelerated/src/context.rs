pub struct Context {
    #[cfg(feature = "reference")]
    pub coco: coco::Suite,

    #[cfg(feature = "c")]
    pub coco_futhark_c: coco_futhark_c::Context,

    #[cfg(feature = "multicore")]
    pub coco_futhark_multicore: coco_futhark_multicore::Context,

    #[cfg(feature = "opencl")]
    pub coco_futhark_opencl: coco_futhark_opencl::Context,

    #[cfg(feature = "cuda")]
    pub coco_futhark_cuda: coco_futhark_cuda::Context,
}

impl Context {
    pub fn new() -> Self {
        Context {
            #[cfg(feature = "reference")]
            coco: coco::Suite::new(coco::SuiteName::Bbob, "", "").unwrap(),

            #[cfg(feature = "c")]
            coco_futhark_c: coco_futhark_c::Context::default(),

            #[cfg(feature = "multicore")]
            coco_futhark_multicore: coco_futhark_multicore::Context::default(),

            #[cfg(feature = "opencl")]
            coco_futhark_opencl: coco_futhark_opencl::Context::default(),

            #[cfg(feature = "cuda")]
            coco_futhark_cuda: coco_futhark_cuda::Context::default(),
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
