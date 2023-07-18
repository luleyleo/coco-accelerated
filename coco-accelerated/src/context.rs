use coco_futhark::backend;

pub struct Context {
    #[cfg(feature = "c")]
    pub coco_futhark_c: coco_futhark::Context<backend::C>,

    #[cfg(feature = "multicore")]
    pub coco_futhark_multicore: coco_futhark::Context<backend::Multicore>,

    #[cfg(feature = "opencl")]
    pub coco_futhark_opencl: coco_futhark::Context<backend::OpenCL>,

    #[cfg(feature = "cuda")]
    pub coco_futhark_cuda: coco_futhark::Context<backend::Cuda>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            #[cfg(feature = "c")]
            coco_futhark_c: coco_futhark::Context::default(),

            #[cfg(feature = "multicore")]
            coco_futhark_multicore: coco_futhark::Context::default(),

            #[cfg(feature = "opencl")]
            coco_futhark_opencl: coco_futhark::Context::default(),

            #[cfg(feature = "cuda")]
            coco_futhark_cuda: coco_futhark::Context::default(),
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
