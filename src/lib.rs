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
    hash_bits: i32
}

#[link(name = "ooz")]
extern "C" {
    pub fn Kraken_Decompress(
        src: *const u8,
        src_len: usize,
        dst: *mut u8,
        dst_len: usize
    ) -> i32;
    pub fn CompressBlock(
        codec_id: i32,
        src_in: *const u8,
        dst_in: *mut u8,
        src_size: i32,
        level: i32,
        compress_options: *const CompressOptions,
        src_window_base: *const u8,
        lrm: *const c_void
    );
}

#[cfg(test)]
mod tests {
    use crate::Kraken_Decompress;

    #[test]
    fn unsafe_decompress() {
        let src = std::fs::read("mermaid-fortnite.dump").unwrap();
        let mut dst = vec![0u8; 405273];
        let result = unsafe { Kraken_Decompress(src.as_ptr(), src.len(), dst.as_mut_ptr(), dst.len()) };
        assert_eq!(result as usize, dst.len());
    }
}
