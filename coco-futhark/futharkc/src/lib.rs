use rerun_except::rerun_except;
use std::{env, fs, path::PathBuf, process::Command};

const CARGO_MANIFEST_DIR: &str = "CARGO_MANIFEST_DIR";

fn source_dir() -> PathBuf {
    let source = PathBuf::from("../shared/src/futhark");

    assert!(source.is_dir(), "src/futhark does not exist");

    source
}

fn source_file() -> PathBuf {
    let source = source_dir().join("batch.fut");

    assert!(source.is_file(), "bbob.fut does not exist");

    source
}

pub fn watch_source() {
    let old_manifest_dir = env::var_os(CARGO_MANIFEST_DIR).unwrap();

    env::set_var(CARGO_MANIFEST_DIR, source_dir().as_os_str());
    rerun_except(&[]).expect("Failed to watch files.");
    env::set_var(CARGO_MANIFEST_DIR, old_manifest_dir);
}

pub fn build_target(compiler: &str) {
    let out_dir = &env::var("OUT_DIR").expect("OUT_DIR is undefined");
    let target = &PathBuf::from(out_dir).join("futhark");
    fs::create_dir_all(&target).expect("Could not create target dir.");

    let source = &source_file();

    let futhark_status = Command::new("futhark")
        .args(&[compiler, "--library", "-o"])
        .arg(target.join("raw"))
        .arg(source)
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success();

    if !futhark_status {
        panic!("Failed to compile Futhark code");
    }

    bindgen::Builder::default()
        .clang_arg("-I/opt/cuda/include")
        .clang_arg("-I/nix/store/g15j0y3fzvx4kkry4viymn698m1gk8yx-cudatoolkit-11.7.0/include")
        .header(target.join("raw.h").to_string_lossy())
        .allowlist_function("free")
        .allowlist_function("futhark_.*")
        .allowlist_type("futhark_.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings.")
        .write_to_file(target.join("raw.rs"))
        .expect("Couldn't write bindings!");

    cc::Build::new()
        .file(target.join("raw.c"))
        .include("/opt/cuda/include")
        .warnings(false)
        .compile(&format!("coco-futhark-{compiler}", compiler = compiler));
}
