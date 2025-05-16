use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug)]
pub enum BuildError {
    /// Failed to find GPU architecture
    FailedToFindGPU(String)
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unable to read configuration at Find GPU")
    }
}

impl Error for BuildError {}

#[cfg(feature = "nvidia")]
fn detect_nvidia_arch() -> Option<String> {
    let output = Command::new("nvidia-smi")
        .args(&["--query-gpu=compute_cap", "--format=csv,noheader"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let cap = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let arch = cap.replace('.', "");
    Some(format!("sm_{}", arch))
}



#[cfg(feature = "nvidia")]
fn build_nvidia() -> Result<(), BuildError> {

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    if let Some(arch) = detect_nvidia_arch() {
        // Compile the CUDA file
        let status = Command::new("nvcc")
            .args(&[format!("-arch={}", arch).as_str(), "-c", "cuda/kernels.cu", "-o"])
            .arg(out_dir.join("kernels.o"))
            .args(&["-Xcompiler", "-fPIC"])
            .status()
            .expect("Failed to invoke nvcc");
        assert!(status.success(), "nvcc failed");


        // c - create a new archive
        // r - insert files into archive, replace existing files
        // u - update the archive, adds new files and updates existing
        // s - create an index (symbol table),
        let status = Command::new("ar")
            .args(&["crus"])
            .arg(out_dir.join("libkernels.a"))
            .arg(out_dir.join("kernels.o"))
            .status()
            .expect("Failed to invoke AR");
            assert!(status.success(), "ar failed");


        // Let cargo know where to find the compiled CUDA library
        println!("cargo:rustc-link-search=native={}", out_dir.display());
        println!("cargo:rustc-link-lib=static=kernels");

        // Link against the CUDA runtime
        println!("cargo:rustc-link-lib=cudart");

        // Add path to CUDA lib directory if not in default search path
        println!("cargo:rustc-link-search=native=/usr/local/cuda/lib64");

        return Ok(())
    } {

        return Err(BuildError::FailedToFindGPU("Failed To Find NVIDIA GPU".to_string()))
    }

}

fn main() {

    #[cfg(feature = "nvidia")]
    {
        build_nvidia().unwrap();
    }

    #[cfg(feature = "amd")]
    {
        // build_amd().unwrap();
    }

}
