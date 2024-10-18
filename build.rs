extern crate bindgen;
extern crate cmake;
use cmake::Config;
use std::path::PathBuf;

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=a_x_b_mod_m/include/oismailov/a_x_b_mod_m.h");
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate bindings for.
        .header("a_x_b_mod_m/include/oismailov/a_x_b_mod_m.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the src/bindings.rs file.
    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Build static library
    let dst = Config::new("a_x_b_mod_m").build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=oismailov");
}
