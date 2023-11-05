fn build_cmake() {
    let dst = cmake::build("c-blosc2");
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-search=native={}/lib64", dst.display());
    // println!("cargo:rustc-link-lib=static=blosc2");
}

#[cfg(feature = "bindgen")]
fn bindgen_rs() {
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("c-blosc2/include/blosc2.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .blocklist_type("__uint64_t_")
        .blocklist_type("__size_t")
        .blocklist_item("BLOSC2_[C|D]PARAMS_DEFAULTS")
        .allowlist_type(".*BLOSC.*")
        .allowlist_type(".*blosc2.*")
        .allowlist_function(".*blosc.*")
        .allowlist_var(".*BLOSC.*")
        .size_t_is_usize(true)
        .no_default("blosc2_[c|d]params")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=c-blosc2");

    build_cmake();

    #[cfg(feature = "bindgen")]
    bindgen_rs();
}
