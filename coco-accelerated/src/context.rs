use std::cell::RefCell;

pub struct Context {
    pub coco: RefCell<coco::Suite>,

    #[cfg(feature = "c")]
    pub futhark_c: coco_futhark_c::Context,

    #[cfg(feature = "multicore")]
    pub futhark_multicore: coco_futhark_multicore::Context,
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
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
