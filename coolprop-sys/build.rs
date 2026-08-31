fn main() {
    #[cfg(feature = "regen-bindings")]
    {
        use std::{env, path::PathBuf};

        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

        println!("cargo:rerun-if-changed=build.rs");
        println!("cargo:rerun-if-changed=CoolPropLib.h");

        fn base_builder() -> bindgen::Builder {
            bindgen::Builder::default()
                .header("CoolPropLib.h")
                .derive_debug(true)
                .derive_default(true)
                .use_core()
                .generate_cstr(true)
                .generate_comments(false)
                .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        }

        let static_bindings = base_builder()
            .generate()
            .expect("bindgen should generate static bindings from `CoolPropLib.h`");

        static_bindings
            .write_to_file(out_dir.join("bindings_static.rs"))
            .expect("static bindings should be written to `OUT_DIR`");

        let dynamic_bindings = base_builder()
            .dynamic_library_name("CoolProp")
            .dynamic_link_require_all(true)
            .generate()
            .expect("bindgen should generate dynamic bindings from `CoolPropLib.h`");

        dynamic_bindings
            .write_to_file(out_dir.join("bindings_dynamic.rs"))
            .expect("dynamic bindings should be written to `OUT_DIR`");
    }
}
