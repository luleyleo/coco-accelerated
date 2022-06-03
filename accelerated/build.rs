use std::{env, fs, path::PathBuf, process::Command};

fn main() {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR is undefined");
    let source = PathBuf::from("src/futhark/raw.fut");
    let target = PathBuf::from(out_dir).join("futhark");

    fs::create_dir_all(&target).expect("Could not create target dir.");

    assert!(source.is_file(), "raw.fut does not exist");

    let futhark_status = Command::new("futhark")
        .args(&["c", "--library", "-o"])
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
        .warnings(false)
        .compile("coco");
}
