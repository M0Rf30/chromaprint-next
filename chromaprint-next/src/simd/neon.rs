use std::arch::aarch64::*;

/// NEON-accelerated i16 × i16 dot product with i32 accumulator.
///
/// Uses `vmlal_s16` (signed multiply-accumulate long) to process
/// 4 i16 pairs per instruction, with two accumulator chains for
/// instruction-level parallelism.
#[inline]
pub fn dot_product_i16(a: &[i16], b: &[i16]) -> i32 {
    debug_assert_eq!(a.len(), b.len());
    let len = a.len();

    unsafe {
        let a_ptr = a.as_ptr();
        let b_ptr = b.as_ptr();

        let mut acc0 = vdupq_n_s32(0);
        let mut acc1 = vdupq_n_s32(0);

        let chunks = len / 8;

        for i in 0..chunks {
            let offset = i * 8;
            let a_lo = vld1_s16(a_ptr.add(offset));
            let b_lo = vld1_s16(b_ptr.add(offset));
            let a_hi = vld1_s16(a_ptr.add(offset + 4));
            let b_hi = vld1_s16(b_ptr.add(offset + 4));

            acc0 = vmlal_s16(acc0, a_lo, b_lo);
            acc1 = vmlal_s16(acc1, a_hi, b_hi);
        }

        acc0 = vaddq_s32(acc0, acc1);

        // Handle 4-element remainder chunk
        let mut offset = chunks * 8;
        if len - offset >= 4 {
            let a_v = vld1_s16(a_ptr.add(offset));
            let b_v = vld1_s16(b_ptr.add(offset));
            acc0 = vmlal_s16(acc0, a_v, b_v);
            offset += 4;
        }

        // Horizontal sum
        let mut result = vaddvq_s32(acc0);

        // Scalar tail (0-3 elements)
        for i in 0..(len - offset) {
            result += *a_ptr.add(offset + i) as i32 * *b_ptr.add(offset + i) as i32;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neon_matches_scalar() {
        for len in [1, 3, 4, 7, 8, 15, 16, 30, 59, 80, 88] {
            let a: Vec<i16> = (0..len).map(|i| (i * 7 - 100) as i16).collect();
            let b: Vec<i16> = (0..len).map(|i| (i * 3 + 50) as i16).collect();

            let scalar: i32 = a.iter().zip(b.iter()).map(|(&x, &y)| x as i32 * y as i32).sum();
            let neon = dot_product_i16(&a, &b);
            assert_eq!(scalar, neon, "mismatch for len={len}");
        }
    }
}
