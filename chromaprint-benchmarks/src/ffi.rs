use std::ffi::c_void;
use std::ptr;

extern "C" {
    fn chromaprint_new(algorithm: i32) -> *mut c_void;
    fn chromaprint_free(ctx: *mut c_void);
    fn chromaprint_start(ctx: *mut c_void, sample_rate: i32, num_channels: i32) -> i32;
    fn chromaprint_feed(ctx: *mut c_void, data: *const i16, size: i32) -> i32;
    fn chromaprint_finish(ctx: *mut c_void) -> i32;
    fn chromaprint_get_raw_fingerprint(
        ctx: *mut c_void,
        fingerprint: *mut *mut u32,
        size: *mut i32,
    ) -> i32;
    fn chromaprint_dealloc(ptr: *mut c_void);
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
        let ret = unsafe { chromaprint_feed(self.ctx, data.as_ptr(), data.len() as i32) };
        assert_eq!(ret, 1, "chromaprint_feed failed");
    }

    pub fn finish(&self) {
        let ret = unsafe { chromaprint_finish(self.ctx) };
        assert_eq!(ret, 1, "chromaprint_finish failed");
    }

    pub fn get_raw_fingerprint(&self) -> Vec<u32> {
        let mut fp: *mut u32 = ptr::null_mut();
        let mut size: i32 = 0;
        let ret = unsafe { chromaprint_get_raw_fingerprint(self.ctx, &mut fp, &mut size) };
        assert_eq!(ret, 1, "chromaprint_get_raw_fingerprint failed");
        let slice = unsafe { std::slice::from_raw_parts(fp, size as usize) };
        let result = slice.to_vec();
        unsafe { chromaprint_dealloc(fp as *mut c_void) };
        result
    }
}

impl Drop for CChromaprint {
    fn drop(&mut self) {
        unsafe { chromaprint_free(self.ctx) };
    }
}
