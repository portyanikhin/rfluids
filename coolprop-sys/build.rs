use std::{env, path::PathBuf};

const LIB_NAME: &str = "CoolProp";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=CoolPropLib.h");
    let target_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings = bindgen::Builder::default()
        .header("CoolPropLib.h")
        .derive_debug(true)
        .derive_default(true)
        .dynamic_library_name(LIB_NAME)
        .dynamic_link_require_all(true)
        .use_core()
        .generate_cstr(true)
        .generate_comments(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings!");
    bindings.write_to_file(target_dir.join("bindings.rs")).expect("Unable to write bindings!");
}
