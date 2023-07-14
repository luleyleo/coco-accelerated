fn main() {
    futharkc::build_target("opencl").unwrap();
    futharkc::watch_source().unwrap();

    println!("cargo:rustc-link-lib=OpenCL");
}
