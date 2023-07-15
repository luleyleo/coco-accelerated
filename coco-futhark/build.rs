use coco_futhark_compiler::{build_target, watch_source};

fn main() {
    build_target("c").unwrap();
    watch_source().unwrap();
}
