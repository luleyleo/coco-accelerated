use coco_futhark_compiler::{build_target, watch_source};

fn main() {
    watch_source().unwrap();

    #[cfg(feature = "c")]
    build_target("c").unwrap();

    #[cfg(feature = "multicore")]
    build_target("multicore").unwrap();
}
