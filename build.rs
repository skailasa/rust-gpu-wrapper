use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    use std::env;
    use std::path::PathBuf;
    use std::process::Command;

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let cuda_lib_path = out_dir.join("libkernels.a");

    // Compile the CUDA file
    Command::new("nvcc")
        .args(&["-arch=sm_89", "-c", "cuda/kernels.cu", "-o"])
        .arg(out_dir.join("kernels.o"))
        .args(&["-Xcompiler", "-fPIC"])
        .status()
        .unwrap();

    Command::new("ar")
        .args(&["crus"])
        .arg(out_dir.join("libkernels.a"))
        .arg(out_dir.join("kernels.o"))
        .status()
        .unwrap();

    // Let cargo know where to find the compiled CUDA library
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=kernels");

    // Link against the CUDA runtime
    println!("cargo:rustc-link-lib=cudart");

    // Add path to CUDA lib directory if not in default search path
    // println!("cargo:rustc-link-search=native=/usr/local/cuda/lib64");
}

