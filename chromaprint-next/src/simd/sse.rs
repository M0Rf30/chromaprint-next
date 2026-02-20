#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
#[cfg(target_arch = "x86")]
use std::arch::x86::*;

/// SSE2-accelerated i16 × i16 dot product with i32 accumulator.
///
/// Uses `_mm_madd_epi16` which multiplies 8 i16 pairs and pairwise-adds
/// the products into 4 i32 results — ideal for this pattern.
#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn dot_product_i16(a: &[i16], b: &[i16]) -> i32 {
    debug_assert_eq!(a.len(), b.len());
    let len = a.len();

    let a_ptr = a.as_ptr();
    let b_ptr = b.as_ptr();

    let mut acc0 = _mm_setzero_si128();
    let mut acc1 = _mm_setzero_si128();

    let chunks = len / 16;

    for i in 0..chunks {
        let offset = i * 16;
        let a0 = _mm_loadu_si128(a_ptr.add(offset) as *const __m128i);
        let b0 = _mm_loadu_si128(b_ptr.add(offset) as *const __m128i);
        let a1 = _mm_loadu_si128(a_ptr.add(offset + 8) as *const __m128i);
        let b1 = _mm_loadu_si128(b_ptr.add(offset + 8) as *const __m128i);

        acc0 = _mm_add_epi32(acc0, _mm_madd_epi16(a0, b0));
        acc1 = _mm_add_epi32(acc1, _mm_madd_epi16(a1, b1));
    }

    acc0 = _mm_add_epi32(acc0, acc1);

    // Handle 8-element remainder chunk
    let mut offset = chunks * 16;
    if len - offset >= 8 {
        let a_v = _mm_loadu_si128(a_ptr.add(offset) as *const __m128i);
        let b_v = _mm_loadu_si128(b_ptr.add(offset) as *const __m128i);
        acc0 = _mm_add_epi32(acc0, _mm_madd_epi16(a_v, b_v));
        offset += 8;
    }

    // Horizontal sum of 4 i32 lanes
    let hi64 = _mm_unpackhi_epi64(acc0, acc0);
    acc0 = _mm_add_epi32(acc0, hi64);
    let hi32 = _mm_shuffle_epi32(acc0, 1);
    let sum = _mm_add_epi32(acc0, hi32);
    let mut result = _mm_cvtsi128_si32(sum);

    // Scalar tail (0-7 elements)
    for i in 0..(len - offset) {
        result += *a_ptr.add(offset + i) as i32 * *b_ptr.add(offset + i) as i32;
    }

    result
}
