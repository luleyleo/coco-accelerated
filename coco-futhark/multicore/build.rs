use rerun_except::rerun_except;
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

include!("../shared/build.rs");

fn main() {
    let out_dir = &env::var("OUT_DIR").expect("OUT_DIR is undefined");
    let source = &PathBuf::from("../shared/src/futhark/bbob.fut");

    assert!(source.is_file(), "bbob.fut does not exist");

    let target = &PathBuf::from(out_dir).join("futhark");
    build_target(source, target, "multicore");

    rerun_except(&[]).expect("Failed to watch files.");
}
