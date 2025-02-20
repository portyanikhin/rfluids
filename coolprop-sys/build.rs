use std::path::{Path, PathBuf};
use std::{env, fs};

const LIB_NAME: &str = "CoolProp";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.h");
    let (target_os, target_arch) = get_target_os_and_arch();
    let src_dir = setup_src_dir(&target_os, &target_arch);
    let target_dir = setup_target_dir(&target_os);
    setup_lib(&target_os, &src_dir, &target_dir);
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-v")
        .derive_debug(true)
        .derive_default(true)
        .dynamic_library_name(LIB_NAME)
        .dynamic_link_require_all(true)
        .use_core()
        .generate_cstr(true)
        .generate_comments(false)
        .rust_edition(bindgen::RustEdition::Edition2021)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings!");
    bindings
        .write_to_file(target_dir.join("bindings.rs"))
        .expect("Unable to write bindings!");
}

#[allow(clippy::enum_variant_names)]
enum OS {
    Windows,
    Linux,
    MacOS,
}

enum Arch {
    X86_64,
    AArch64,
}

fn get_target_os_and_arch() -> (OS, Arch) {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    match (target_os.as_str(), target_arch.as_str()) {
        ("windows", "x86_64") => (OS::Windows, Arch::X86_64),
        ("linux", "x86_64") => (OS::Linux, Arch::X86_64),
        ("macos", "x86_64") => (OS::MacOS, Arch::X86_64),
        ("macos", "aarch64") => (OS::MacOS, Arch::AArch64),
        _ => panic!(
            "Unsupported target platform: {} ({})",
            target_os, target_arch
        ),
    }
}

fn setup_src_dir(target_os: &OS, target_arch: &Arch) -> PathBuf {
    let subfolder = match (target_os, target_arch) {
        (OS::Windows, Arch::X86_64) => "win-x86-64",
        (OS::Linux, Arch::X86_64) => "lin-x86-64",
        (OS::MacOS, Arch::X86_64) => "mac-x86-64",
        (OS::MacOS, Arch::AArch64) => "mac-aarch64",
        _ => "",
    };
    let src_dir = PathBuf::from(format!("native/lib/{}", subfolder))
        .canonicalize()
        .expect("Unable to canonicalize CoolProp directory path!");
    println!(
        "cargo:rustc-link-search=native={}",
        src_dir.to_str().unwrap()
    );
    src_dir
}

fn setup_target_dir(target_os: &OS) -> PathBuf {
    let target_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!(
        "cargo:rustc-link-search=native={}",
        target_dir.to_str().unwrap()
    );
    if matches!(target_os, OS::Linux | OS::MacOS) {
        println!(
            "cargo:rustc-link-arg=-Wl,-rpath,{}",
            target_dir.to_str().unwrap()
        );
    }
    target_dir
}

fn setup_lib(target_os: &OS, src_dir: &Path, target_dir: &Path) {
    let prefix = if matches!(target_os, OS::Windows) {
        ""
    } else {
        "lib"
    };
    let lib_extension = match target_os {
        OS::Windows => ".dll",
        OS::Linux => ".so",
        OS::MacOS => ".dylib",
    };
    let file_name = format!("{}{}{}", prefix, LIB_NAME, lib_extension);
    let src_path = src_dir.join(&file_name);
    let target_path = target_dir.join(&file_name);
    fs::copy(&src_path, &target_path)
        .expect("Unable to copy CoolProp library to the target directory!");
    println!("cargo:rustc-link-lib=dylib={}", LIB_NAME);
}
