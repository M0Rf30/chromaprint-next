#[cfg(not(target_arch = "aarch64"))]
mod scalar;

#[cfg(target_arch = "aarch64")]
mod neon;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod sse;

/// Compute the dot product of two i16 slices, returning an i32 accumulator.
///
/// Dispatches to NEON on aarch64, SSE2 on x86/x86_64, or scalar fallback.
/// Integer SIMD produces identical results to scalar — no rounding concerns.
#[inline]
pub fn dot_product_i16(a: &[i16], b: &[i16]) -> i32 {
    #[cfg(target_arch = "aarch64")]
    {
        neon::dot_product_i16(a, b)
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("sse2") {
            // SAFETY: we just verified SSE2 is available
            unsafe { sse::dot_product_i16(a, b) }
        } else {
            scalar::dot_product_i16(a, b)
        }
    }

    #[cfg(not(any(target_arch = "aarch64", target_arch = "x86", target_arch = "x86_64")))]
    {
        scalar::dot_product_i16(a, b)
    }
}

/// Mix interleaved stereo i16 pairs to mono by averaging.
///
/// `input` has `output.len() * 2` elements (L0,R0,L1,R1,...).
/// output[i] = (input[2*i] + input[2*i+1]) / 2 (truncation toward zero).
#[inline]
pub fn stereo_to_mono(input: &[i16], output: &mut [i16]) {
    #[cfg(target_arch = "aarch64")]
    {
        neon::stereo_to_mono(input, output)
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("sse2") {
            unsafe { sse::stereo_to_mono(input, output) }
        } else {
            scalar::stereo_to_mono(input, output)
        }
    }

    #[cfg(not(any(target_arch = "aarch64", target_arch = "x86", target_arch = "x86_64")))]
    {
        scalar::stereo_to_mono(input, output)
    }
}
