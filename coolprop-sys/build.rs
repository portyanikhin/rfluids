use std::path::PathBuf;
use std::{env, fs};

fn main() {
    let (target_os, target_arch) = get_target_os_and_arch();
    let lib_dir = get_lib_dir(&target_os, &target_arch);
    let target_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", lib_dir.to_str().unwrap());
    println!("cargo:rustc-link-search={}", target_dir.to_str().unwrap());
    if matches!(target_os, OS::Linux | OS::MacOS) {
        println!(
            "cargo:rustc-link-arg=-Wl,-rpath,{}",
            target_dir.to_str().unwrap()
        );
    }
    copy_lib_to_target_dir(&lib_dir, &target_dir, &target_os);
    println!("cargo:rerun-if-changed=wrapper.h");
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-v")
        .derive_debug(true)
        .derive_default(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .use_core()
        .generate()
        .expect("Unable to generate bindings!");
    bindings
        .write_to_file(target_dir.join("bindings.rs"))
        .expect("Unable to write bindings!");
}

enum OS {
    Windows,
    Linux,
    MacOS,
}

enum Arch {
    X64,
    ARM64,
}

fn get_target_os_and_arch() -> (OS, Arch) {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    match (target_os.as_str(), target_arch.as_str()) {
        ("windows", "x86_64") => (OS::Windows, Arch::X64),
        ("linux", "x86_64") => (OS::Linux, Arch::X64),
        ("macos", "x86_64") => (OS::MacOS, Arch::X64),
        ("macos", "aarch64") => (OS::MacOS, Arch::ARM64),
        _ => panic!(
            "Unsupported target platform: {} ({})",
            target_os, target_arch
        ),
    }
}

fn get_lib_dir(target_os: &OS, target_arch: &Arch) -> PathBuf {
    let subfolder = match (target_os, target_arch) {
        (OS::Windows, Arch::X64) => "win-x64",
        (OS::Linux, Arch::X64) => "lin-x64",
        (OS::MacOS, Arch::X64) => "mac-x64",
        (OS::MacOS, Arch::ARM64) => "mac-arm64",
        _ => "",
    };
    PathBuf::from(format!("native/lib/{}", subfolder))
        .canonicalize()
        .expect("Unable to canonicalize CoolProp directory path!")
}

fn copy_lib_to_target_dir(lib_dir: &PathBuf, target_dir: &PathBuf, target_os: &OS) {
    let lib_name = if matches!(target_os, OS::Windows) {
        "CoolProp"
    } else {
        "libCoolProp"
    };
    let lib_extension = match target_os {
        OS::Windows => ".dll",
        OS::Linux => ".so",
        OS::MacOS => ".dylib",
    };
    let lib_file_name = format!("{}{}", lib_name, lib_extension);
    println!("cargo:rustc-link-lib={}", lib_name);
    fs::copy(
        lib_dir.join(&lib_file_name),
        target_dir.join(&lib_file_name),
    )
    .expect("Unable to copy CoolProp library to the target directory!");
}
