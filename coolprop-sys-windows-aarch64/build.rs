use std::{
    env, fs,
    path::{Path, PathBuf},
};

const LIB_PREFIX: &str = "";
const LIB_NAME: &str = "CoolProp";
const LIB_EXTENSION: &str = ".dll";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap().to_lowercase();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap().to_lowercase();
    if target_os == "windows" && target_arch == "aarch64" {
        let src_dir = setup_src_dir();
        let target_dir = setup_target_dir();
        setup_lib(&src_dir, &target_dir);
    }
}

fn setup_src_dir() -> PathBuf {
    let src_dir = PathBuf::from("lib")
        .canonicalize()
        .expect("Unable to canonicalize CoolProp directory path!");
    println!("cargo:rustc-link-search=native={}", src_dir.to_str().unwrap());
    src_dir
}

fn setup_target_dir() -> PathBuf {
    let target_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search=native={}", target_dir.to_str().unwrap());
    target_dir
}

fn setup_lib(src_dir: &Path, target_dir: &Path) {
    let file_name = format!("{}{}{}", LIB_PREFIX, LIB_NAME, LIB_EXTENSION);
    let src_path = src_dir.join(&file_name);
    let target_path = target_dir.join(&file_name);
    fs::copy(&src_path, &target_path)
        .expect("Unable to copy CoolProp library to the target directory!");
    println!("cargo:rustc-link-lib=dylib={}", LIB_NAME);
}
