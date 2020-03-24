use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=csrc/my_c_lib.c");
    println!("cargo:rerun-if-changed=csrc/my_c_lib.h");
    println!("cargo:rerun-if-changed=src/lib.rs");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .generate()
        .expect("Could not generate C bindings")
        .write_to_file(out_path.join("bindings.h"));

    cc::Build::new()
        .file("csrc/my_c_lib.c")
        .include(&out_path)
        .opt_level(2)
        .compile("my_c_lib");

    bindgen::Builder::default()
        .header("csrc/my_c_lib.h")
        .generate()
        .expect("Could not generate Rust bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
