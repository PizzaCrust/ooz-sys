use std::ffi::c_void;

#[repr(C)]
pub struct CompressOptions {
    unknown_0: i32,
    min_match_length: i32,
    seek_chunk_reset: i32,
    seek_chunk_len: i32,
    unknown_1: i32,
    dictionary_size: i32,
    space_speed_tradeoff_bytes: i32,
    unknown_2: i32,
    make_qhcrc: i32,
    max_local_dictionary_size: i32,
    make_long_range_matcher: i32,
    hash_bits: i32,
}

#[repr(i32)]
pub enum Compressor {
    Kraken = 8,
    Mermaid = 9,
    Selkie = 11,
    Leviathan = 13,
}

#[repr(i32)]
pub enum CompressorLevel {
    None,
    SuperFast,
    VeryFast,
    Fast,
    Normal,
    Optimal1,
    Optimal2,
    Optimal3,
    Optimal4,
    Optimal5,
}

#[link(name = "ooz")]
extern "C" {
    pub fn Kraken_Decompress(src: *const u8, src_len: usize, dst: *mut u8, dst_len: usize) -> i32;
    pub fn Compress(
        codec_id: Compressor,
        src_in: *const u8,
        dst_in: *mut u8,
        src_size: i32,
        level: CompressorLevel,
        compress_options: *const CompressOptions,
        src_window_base: *const u8,
        lrm: *const c_void,
    ) -> i32;
}

#[cfg(test)]
mod tests {
    use crate::{Compress, Compressor, CompressorLevel, Kraken_Decompress};
    use std::ptr;

    #[test]
    fn unsafe_decompress() {
        let src = include_bytes!("../mermaid-fortnite.dump");
        let mut dst = vec![0u8; 405273];
        let result =
            unsafe { Kraken_Decompress(src.as_ptr(), src.len(), dst.as_mut_ptr(), dst.len()) };
        assert_eq!(result as usize, dst.len());
    }

    #[test]
    fn unsafe_compress() {
        let src = [1u8; 10_000];
        let mut dst = vec![0u8; src.len() + 65536];
        let result = unsafe {
            Compress(
                Compressor::Kraken,
                src.as_ptr(),
                dst.as_mut_ptr(),
                src.len() as i32,
                CompressorLevel::Optimal1,
                ptr::null(),
                ptr::null(),
                ptr::null(),
            )
        };
        assert!(result > 0);
    }
}
