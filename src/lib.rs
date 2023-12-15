//! This crate is for building `c-blosc` and linking to the static build.

#![allow(clippy::redundant_static_lifetimes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::{ffi::c_void, ptr::null_mut};

pub use libc::FILE;

#[cfg(not(windows))]
pub use libc::timespec;

#[cfg(windows)]
pub type LARGE_INTEGER = i64;

#[cfg(not(feature = "bindgen"))]
include!("bindings.rs");
#[cfg(feature = "bindgen")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// Defaults mirror BLOSC2_CPARAMS_DEFAULTS in blosc2.h
///
/// static const values are compile time constructs, and can't have bindings
/// generated to them.
pub const BLOSC2_CPARAMS_DEFAULTS: blosc2_cparams = blosc2_cparams {
    compcode: BLOSC_BLOSCLZ as u8,
    compcode_meta: 0,
    clevel: 5,
    use_dict: 0,
    typesize: 8,
    nthreads: 1,
    blocksize: 0,
    splitmode: BLOSC_FORWARD_COMPAT_SPLIT as std::os::raw::c_int,
    schunk: null_mut::<c_void>(),
    filters: [0, 0, 0, 0, 0, BLOSC_SHUFFLE as u8],
    filters_meta: [0, 0, 0, 0, 0, 0],
    prefilter: None,
    preparams: null_mut::<blosc2_prefilter_params>(),
    tuner_params: null_mut::<c_void>(),
    tuner_id: 0,
    instr_codec: false,
    codec_params: null_mut::<c_void>(),
    filter_params: [
        null_mut::<c_void>(),
        null_mut::<c_void>(),
        null_mut::<c_void>(),
        null_mut::<c_void>(),
        null_mut::<c_void>(),
        null_mut::<c_void>(),
    ],
};

impl Default for blosc2_cparams {
    fn default() -> Self {
        BLOSC2_CPARAMS_DEFAULTS
    }
}

/// Defaults mirror BLOSC2_DPARAMS_DEFAULTS in blosc2.h
///
/// static const values are compile time constructs, and can't have bindings
/// generated to them.
pub const BLOSC2_DPARAMS_DEFAULTS: blosc2_dparams = blosc2_dparams {
    nthreads: 1,
    schunk: null_mut::<c_void>(),
    postfilter: None,
    postparams: null_mut::<blosc2_postfilter_params>(),
};

impl Default for blosc2_dparams {
    fn default() -> Self {
        BLOSC2_DPARAMS_DEFAULTS
    }
}
