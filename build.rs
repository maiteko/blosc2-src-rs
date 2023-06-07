use std::fs;

use cc::Build;

fn build_cc() {
    let mut build = cc::Build::new();

    let target_mscv = cfg!(target_env = "msvc");
    let add_file = |builder: &mut Build, folder: &str| {
        for entry in fs::read_dir(folder).unwrap() {
            let path = entry.unwrap().path();
            if let Some(extension) = path.extension() {
                if extension == "c" || extension == "cpp" || (!target_mscv && extension == "S") {
                    builder.file(path);
                }
            }
        }
    };

    build.include("c-blosc2/include");
    add_file(&mut build, "c-blosc2/blosc");
    add_file(&mut build, "c-blosc2/internal-complibs/lz4-1.9.4");
    add_file(&mut build, "c-blosc2/include");

    build.include("c-blosc2/internal-complibs/lz4-1.9.4");
    build.define("HAVE_LZ4", None);
    if cfg!(feature = "zlib") {
        add_file(&mut build, "c-blosc2/internal-complibs/zlib-1.2.11");
        build.include("c-blosc2/internal-complibs/zlib-1.2.11");
        build.define("HAVE_ZLIB", None);
    }
    if cfg!(feature = "zstd") {
        add_file(&mut build, "c-blosc2/internal-complibs/zstd-1.5.2/common");
        add_file(&mut build, "c-blosc2/internal-complibs/zstd-1.5.2/compress");
        add_file(
            &mut build,
            "c-blosc2/internal-complibs/zstd-1.5.2/decompress",
        );
        add_file(
            &mut build,
            "c-blosc2/internal-complibs/zstd-1.5.2/dictBuilder",
        );
        build.include("c-blosc2/internal-complibs/zstd-1.5.2");
        build.define("HAVE_ZSTD", None);
    }

    build.compile("blosc2");
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

    build_cc();

    #[cfg(feature = "bindgen")]
    bindgen_rs();
}
