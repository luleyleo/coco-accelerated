use bindgen::callbacks::ParseCallbacks;
use eyre::{bail, ensure, Context, ContextCompat, Result};
use rerun_except::rerun_except;
use std::{
    env,
    fs::{self, File},
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    process::Command,
};

const CARGO_MANIFEST_DIR: &str = "CARGO_MANIFEST_DIR";
const FUTHARK_SOURCE_FILE: &str = "batch.fut";
const SOURCE_DIR: &str = "./src/futhark";

fn main() -> Result<()> {
    watch_source().wrap_err("Failed to watch source files for changes.")?;

    #[cfg(feature = "c")]
    build_target("c").wrap_err("Failed to build C target.")?;

    #[cfg(feature = "multicore")]
    build_target("multicore").wrap_err("Failed to build Multi-Core target.")?;

    #[cfg(feature = "opencl")]
    {
        build_target("opencl").wrap_err("Failed to build OpenCL target.")?;

        println!("cargo:rustc-link-lib=OpenCL");
    }

    #[cfg(feature = "cuda")]
    {
        futharkc::build_target("cuda").wrap_err("Failed to build Cuda target.")?;

        println!("cargo:rustc-link-search=/opt/cuda/lib64");
        println!("cargo:rustc-link-lib=cuda");
        println!("cargo:rustc-link-lib=cudart");
        println!("cargo:rustc-link-lib=nvrtc");
    }

    Ok(())
}

fn watch_source() -> Result<()> {
    let old_manifest_dir = env::var_os(CARGO_MANIFEST_DIR)
        .wrap_err("CARGO_MANIFEST_DIR environment variable is not defined.")?;

    env::set_var(CARGO_MANIFEST_DIR, PathBuf::from(SOURCE_DIR).as_os_str());

    rerun_except(&[])
        .map_err(|err| eyre::eyre!("{}", err))
        .wrap_err("Failed to watch files.")?;

    env::set_var(CARGO_MANIFEST_DIR, old_manifest_dir);

    Ok(())
}

fn build_target(compiler: &str) -> Result<()> {
    let out_dir = &env::var("OUT_DIR").wrap_err("OUT_DIR is undefined.")?;

    let target_dir = &PathBuf::from(out_dir).join("futhark").join(compiler);
    fs::create_dir_all(&target_dir).wrap_err("Could not create target dir.")?;

    let raw_target_dir = &PathBuf::from(out_dir).join("futhark_raw").join(compiler);
    fs::create_dir_all(&raw_target_dir).wrap_err("Could not create raw target dir.")?;

    let source = PathBuf::from(SOURCE_DIR).join(FUTHARK_SOURCE_FILE);
    ensure!(source.is_file(), "Futhark source file does not exist.");

    let futhark_status = Command::new("futhark")
        .args(&[compiler, "--library", "-o"])
        .arg(raw_target_dir.join("accelerated"))
        .arg(source)
        .status()
        .wrap_err("Failed to run Futhark compiler.")?
        .success();

    if !futhark_status {
        bail!("Failed to compile Futhark code.");
    }

    let prefix = format!("futhark_{compiler}_");

    prefix_items(
        &prefix,
        raw_target_dir.join("accelerated.h"),
        target_dir.join("accelerated.h"),
    )
    .wrap_err("Failed to prefix header file items.")?;

    prefix_items(
        &prefix,
        raw_target_dir.join("accelerated.c"),
        target_dir.join("accelerated.c"),
    )
    .wrap_err("Failed to prefix source file items.")?;

    bindgen::Builder::default()
        .clang_arg("-I/opt/cuda/include")
        .header(target_dir.join("accelerated.h").to_string_lossy())
        .allowlist_function("free")
        .allowlist_function("futhark_.*")
        .allowlist_type("futhark_.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .parse_callbacks(PrefixRemover::new(prefix))
        .generate()
        .wrap_err("Failed to generate bindings.")?
        .write_to_file(target_dir.join("accelerated.rs"))
        .wrap_err("Failed to write bindings to file.")?;

    cc::Build::new()
        .file(target_dir.join("accelerated.c"))
        .include("/opt/cuda/include")
        .static_flag(true)
        .warnings(false)
        .try_compile(&format!("coco-futhark-{compiler}", compiler = compiler))
        .wrap_err("Failed to compile the generated c code.")?;

    Ok(())
}

fn prefix_items(prefix: &str, input: impl AsRef<Path>, output: impl AsRef<Path>) -> Result<()> {
    let mut out = BufWriter::new(File::create(output).wrap_err("Failed to open output file.")?);

    let memblock_prefix = &format!("{prefix}_memblock_");
    let lexical_realloc_error_prefix = &format!("{prefix}_lexical_realloc_error");

    for line in fs::read_to_string(input)?.lines() {
        let new_line = line
            .replace("memblock_", memblock_prefix)
            .replace("lexical_realloc_error", lexical_realloc_error_prefix)
            .replace("futhark_", &prefix);

        writeln!(out, "{}", new_line).wrap_err("Failed to write line to output file.")?;
    }

    out.flush().wrap_err("Failed to flush output file.")?;

    Ok(())
}

#[derive(Debug)]
struct PrefixRemover {
    prefix: String,
}

impl PrefixRemover {
    pub fn new(prefix: impl ToOwned<Owned = String>) -> Box<dyn ParseCallbacks> {
        Box::new(PrefixRemover {
            prefix: prefix.to_owned(),
        })
    }
}

impl ParseCallbacks for PrefixRemover {
    fn item_name(&self, original_item_name: &str) -> Option<String> {
        if original_item_name.contains(&self.prefix) {
            return Some(original_item_name.replace(&self.prefix, "futhark_"));
        }

        None
    }
}
