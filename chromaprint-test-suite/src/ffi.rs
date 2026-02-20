use std::ffi::{c_char, c_int, c_void, CStr};
use std::ptr;

extern "C" {
    fn chromaprint_new(algorithm: c_int) -> *mut c_void;
    fn chromaprint_free(ctx: *mut c_void);
    fn chromaprint_start(ctx: *mut c_void, sample_rate: c_int, num_channels: c_int) -> c_int;
    fn chromaprint_feed(ctx: *mut c_void, data: *const i16, size: c_int) -> c_int;
    fn chromaprint_finish(ctx: *mut c_void) -> c_int;
    fn chromaprint_get_fingerprint(ctx: *mut c_void, fingerprint: *mut *mut c_char) -> c_int;
    fn chromaprint_get_raw_fingerprint(
        ctx: *mut c_void,
        fingerprint: *mut *mut u32,
        size: *mut c_int,
    ) -> c_int;
    fn chromaprint_get_fingerprint_hash(ctx: *mut c_void, hash: *mut u32) -> c_int;
    fn chromaprint_dealloc(ptr: *mut c_void);

    fn chromaprint_encode_fingerprint(
        fp: *const u32,
        size: c_int,
        algorithm: c_int,
        encoded_fp: *mut *mut c_char,
        encoded_size: *mut c_int,
        base64: c_int,
    ) -> c_int;

    fn chromaprint_decode_fingerprint(
        encoded_fp: *const c_char,
        encoded_size: c_int,
        fp: *mut *mut u32,
        size: *mut c_int,
        algorithm: *mut c_int,
        base64: c_int,
    ) -> c_int;

    fn chromaprint_hash_fingerprint(
        fp: *const u32,
        size: c_int,
        hash: *mut u32,
    ) -> c_int;
}

/// Safe wrapper around the C chromaprint context.
pub struct CChromaprint {
    ctx: *mut c_void,
}

impl CChromaprint {
    pub fn new(algorithm: i32) -> Self {
        let ctx = unsafe { chromaprint_new(algorithm) };
        assert!(!ctx.is_null(), "chromaprint_new returned null");
        Self { ctx }
    }

    pub fn start(&self, sample_rate: i32, num_channels: i32) {
        let ret = unsafe { chromaprint_start(self.ctx, sample_rate, num_channels) };
        assert_eq!(ret, 1, "chromaprint_start failed");
    }

    pub fn feed(&self, data: &[i16]) {
        let ret = unsafe { chromaprint_feed(self.ctx, data.as_ptr(), data.len() as c_int) };
        assert_eq!(ret, 1, "chromaprint_feed failed");
    }

    pub fn finish(&self) {
        let ret = unsafe { chromaprint_finish(self.ctx) };
        assert_eq!(ret, 1, "chromaprint_finish failed");
    }

    pub fn get_fingerprint(&self) -> String {
        let mut fp: *mut c_char = ptr::null_mut();
        let ret = unsafe { chromaprint_get_fingerprint(self.ctx, &mut fp) };
        assert_eq!(ret, 1, "chromaprint_get_fingerprint failed");
        let s = unsafe { CStr::from_ptr(fp) }.to_string_lossy().into_owned();
        unsafe { chromaprint_dealloc(fp as *mut c_void) };
        s
    }

    pub fn get_raw_fingerprint(&self) -> Vec<u32> {
        let mut fp: *mut u32 = ptr::null_mut();
        let mut size: c_int = 0;
        let ret = unsafe { chromaprint_get_raw_fingerprint(self.ctx, &mut fp, &mut size) };
        assert_eq!(ret, 1, "chromaprint_get_raw_fingerprint failed");
        let slice = unsafe { std::slice::from_raw_parts(fp, size as usize) };
        let result = slice.to_vec();
        unsafe { chromaprint_dealloc(fp as *mut c_void) };
        result
    }

    pub fn get_fingerprint_hash(&self) -> u32 {
        let mut hash: u32 = 0;
        let ret = unsafe { chromaprint_get_fingerprint_hash(self.ctx, &mut hash) };
        assert_eq!(ret, 1, "chromaprint_get_fingerprint_hash failed");
        hash
    }
}

impl Drop for CChromaprint {
    fn drop(&mut self) {
        unsafe { chromaprint_free(self.ctx) };
    }
}

/// Encode a raw fingerprint using the C library.
pub fn c_encode_fingerprint(fp: &[u32], algorithm: i32, base64: bool) -> Vec<u8> {
    let mut encoded: *mut c_char = ptr::null_mut();
    let mut encoded_size: c_int = 0;
    let ret = unsafe {
        chromaprint_encode_fingerprint(
            fp.as_ptr(),
            fp.len() as c_int,
            algorithm,
            &mut encoded,
            &mut encoded_size,
            if base64 { 1 } else { 0 },
        )
    };
    assert_eq!(ret, 1, "chromaprint_encode_fingerprint failed");
    let slice = unsafe { std::slice::from_raw_parts(encoded as *const u8, encoded_size as usize) };
    let result = slice.to_vec();
    unsafe { chromaprint_dealloc(encoded as *mut c_void) };
    result
}

/// Decode an encoded fingerprint using the C library.
pub fn c_decode_fingerprint(encoded: &[u8], base64: bool) -> Option<(Vec<u32>, i32)> {
    let mut fp: *mut u32 = ptr::null_mut();
    let mut size: c_int = 0;
    let mut algorithm: c_int = 0;
    let ret = unsafe {
        chromaprint_decode_fingerprint(
            encoded.as_ptr() as *const c_char,
            encoded.len() as c_int,
            &mut fp,
            &mut size,
            &mut algorithm,
            if base64 { 1 } else { 0 },
        )
    };
    if ret != 1 {
        return None;
    }
    let slice = unsafe { std::slice::from_raw_parts(fp, size as usize) };
    let result = slice.to_vec();
    unsafe { chromaprint_dealloc(fp as *mut c_void) };
    Some((result, algorithm))
}

/// Compute simhash using the C library.
pub fn c_hash_fingerprint(fp: &[u32]) -> u32 {
    let mut hash: u32 = 0;
    let ret = unsafe { chromaprint_hash_fingerprint(fp.as_ptr(), fp.len() as c_int, &mut hash) };
    assert_eq!(ret, 1, "chromaprint_hash_fingerprint failed");
    hash
}
