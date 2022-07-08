fn main() {
    futharkc::build_target("opencl");
    futharkc::watch_source();

    println!("cargo:rustc-link-lib=OpenCL");
}
