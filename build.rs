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

    if cfg!(target_feature = "sse2") {
        build.define("SHUFFLE_SSE2_ENABLED", "1");
        if cfg!(target_env = "msvc") {
            if cfg!(target_pointer_width = "32") {
                build.flag("/arch:SSE2");
            }
        } else {
            build.flag("-msse2");
        }
    }

    if cfg!(target_feature = "avx2") {
        build.define("SHUFFLE_AVX2_ENABLED", "1");
        if cfg!(target_env = "msvc") {
            build.flag("/arch:AVX2");
        } else {
            build.flag("-mavx2");
        }
    }

    {
        // lz4 was optional in cblosc, but seems to be required in cblosc 2
        add_file(&mut build, "c-blosc2/internal-complibs/lz4-1.10.0");
        build.include("c-blosc2/internal-complibs/lz4-1.10.0");
        build.define("HAVE_LZ4", None);
    }
    if cfg!(feature = "zstd") {
        add_file(&mut build, "c-blosc2/internal-complibs/zstd-1.5.6/common");
        add_file(&mut build, "c-blosc2/internal-complibs/zstd-1.5.6/compress");
        add_file(
            &mut build,
            "c-blosc2/internal-complibs/zstd-1.5.6/decompress",
        );
        add_file(
            &mut build,
            "c-blosc2/internal-complibs/zstd-1.5.6/dictBuilder",
        );
        build.include("c-blosc2/internal-complibs/zstd-1.5.6");
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
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .blocklist_type("__uint64_t_")
        .blocklist_type("__size_t")
        .blocklist_item("BLOSC2_[C|D]PARAMS_DEFAULTS")
        .allowlist_type(".*BLOSC.*")
        .allowlist_type(".*blosc2.*")
        .allowlist_function(".*blosc.*")
        .allowlist_var(".*BLOSC.*")
        // Replaced by libc::FILE
        .blocklist_type("FILE")
        .blocklist_type("_IO_FILE")
        .blocklist_type("_IO_codecvt")
        .blocklist_type("_IO_wide_data")
        .blocklist_type("_IO_marker")
        .blocklist_type("_IO_lock_t")
        // Replaced by i64
        .blocklist_type("LARGE_INTEGER")
        // Replaced by libc::timespec
        .blocklist_type("timespec")
        // etc
        .blocklist_type("__time_t")
        .blocklist_type("__syscall_slong_t")
        .blocklist_type("__off64_t")
        .blocklist_type("__off_t")
        .size_t_is_usize(true)
        .no_default("blosc2_[c|d]params")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    build_cc();

    #[cfg(feature = "bindgen")]
    bindgen_rs();
}
