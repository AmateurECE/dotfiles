use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rustc-link-lib=wireplumber-0.5");
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I/usr/include/wireplumber-0.5")
        .clang_arg("-I/usr/include/glib-2.0")
        .clang_arg("-I/usr/lib64/glib-2.0/include")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
