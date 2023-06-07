use blosc2_src::*;

#[test]
fn roundtrip() {
    unsafe {
        blosc2_init();

        let text =
            "I am here writing some very cool and novel words which I will compress and decompress";

        let bytes = text.as_bytes();

        let mut compressed = vec![0; bytes.len() * 2];

        let stat = blosc2_compress(
            9,
            BLOSC_NOSHUFFLE as _,
            std::mem::size_of::<u8>() as i32,
            bytes.as_ptr().cast(),
            bytes.len() as i32,
            compressed.as_mut_ptr().cast(),
            compressed.len() as i32,
        );
        assert!(stat > 0);

        let mut outtext = vec![0_u8; bytes.len()];
        let stat = blosc2_decompress(
            compressed.as_ptr().cast(),
            compressed.len() as i32,
            outtext.as_mut_ptr().cast(),
            outtext.len() as i32,
        );
        assert!(stat > 0);

        assert_eq!(text, std::str::from_utf8(&outtext).unwrap());

        blosc2_destroy();
    }
}

#[test]
fn floats_roundtrip() {
    // generate numerical data
    let src: Vec<f32> = (0..10000)
        .map(|num| ((num * 8923) % 100) as f32 / 2f32) // multiply by big prime number
        .collect();

    // compress
    let dest: Vec<u8> = {
        let typesize = std::mem::size_of::<f32>();
        let src_size = src.len() * typesize;
        let dest_size = src_size + BLOSC2_MAX_OVERHEAD as usize;
        let mut dest = vec![0; dest_size];

        let rsize = unsafe {
            let mut cparams = BLOSC2_CPARAMS_DEFAULTS;
            cparams.typesize = typesize as i32;
            let context = blosc2_create_cctx(cparams);

            // blosc2_compress_ctx(
            //     context,
            //     BLOSC_BITSHUFFLE as i32,
            //     typesize,
            //     src_size,
            //     src.as_ptr().cast(),
            //     dest.as_mut_ptr().cast(),
            //     dest_size,
            //     BLOSC_BLOSCLZ_COMPNAME.as_ptr().cast(),
            //     0,
            //     1,
            // )

            blosc2_compress_ctx(
                context,
                src.as_ptr().cast(),
                src_size as i32,
                dest.as_mut_ptr().cast(),
                dest_size as i32,
            )
        };

        assert!(rsize > 0);
        dest.drain(rsize as usize..);
        dest
    };

    // make sure it actually compresses
    assert!(src.len() * std::mem::size_of::<f32>() > dest.len());

    // decompress
    let result = {
        let mut nbytes: i32 = 0;
        let mut _cbytes: i32 = 0;
        let mut _blocksize: i32 = 0;
        unsafe {
            blosc2_cbuffer_sizes(
                dest.as_ptr().cast(),
                &mut nbytes,
                &mut _cbytes,
                &mut _blocksize,
            )
        };
        assert!(nbytes != 0);
        let dest_size = nbytes / std::mem::size_of::<f32>() as i32;
        let mut result = vec![0f32; dest_size as usize];
        let error = unsafe {
            let dparams = BLOSC2_DPARAMS_DEFAULTS;
            let context = blosc2_create_dctx(dparams);
            blosc2_decompress_ctx(
                context,
                dest.as_ptr().cast(),
                dest.len() as i32,
                result.as_mut_ptr().cast(),
                nbytes,
            )
        };
        assert!(error >= 1);
        result
    };

    // check if the values in both arrays are equal
    assert_eq!(src, result);
}
