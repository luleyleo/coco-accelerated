use coco_futhark_compiler::{build_target, watch_source};

fn main() {
    watch_source().unwrap();

    #[cfg(feature = "c")]
    build_target("c").unwrap();

    #[cfg(feature = "multicore")]
    build_target("multicore").unwrap();

    #[cfg(feature = "opencl")]
    {
        build_target("opencl").unwrap();
        println!("cargo:rustc-link-lib=OpenCL");
    }

    #[cfg(feature = "cuda")]
    {
        futharkc::build_target("cuda").unwrap();

        println!("cargo:rustc-link-search=/opt/cuda/lib64");
        println!("cargo:rustc-link-lib=cuda");
        println!("cargo:rustc-link-lib=cudart");
        println!("cargo:rustc-link-lib=nvrtc");
    }
}
