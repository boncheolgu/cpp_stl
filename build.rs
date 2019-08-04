extern crate bindgen;
extern crate cpp_build;

use std::env;
use std::path::PathBuf;

fn import_stl() {
    use bindgen::*;

    let bindings = Builder::default()
        .whitelist_recursively(false)
        .prepend_enum_name(false)
        .impl_debug(true)
        .with_codegen_config(CodegenConfig::TYPES)
        .layout_tests(false)
        .enable_cxx_namespaces()
        .derive_default(true)
        .whitelist_type("std::string")
        .opaque_type("std::string")
        .whitelist_type("rust::.+")
        .opaque_type("rust::.+")
        .blacklist_type("std")
        .default_enum_style(EnumVariation::Rust {
            non_exhaustive: false,
        })
        .header("csrc/wrapper.hpp")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++11")
        // required to get cross compilation for aarch64 to work because of an issue in flatbuffers
        .clang_arg("-fms-extensions");

    let bindings = bindings.generate().expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/tflite_types.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("stl.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}

fn build_inline_cpp() {
    cpp_build::Config::new()
        .flag("-fPIC")
        .flag("-std=c++11")
        .flag("-Wno-sign-compare")
        .include("csrc")
        .debug(true)
        .opt_level(if cfg!(debug_assertions) { 0 } else { 2 })
        .build("src/lib.rs");
}

fn main() {
    import_stl();
    build_inline_cpp();
}
