fn main() {
    futharkc::build_target("cuda");
    futharkc::watch_source();

    println!("cargo:rustc-link-search=/opt/cuda/lib64");
    println!("cargo:rustc-link-lib=cuda");
    println!("cargo:rustc-link-lib=cudart");
    println!("cargo:rustc-link-lib=nvrtc");
}
