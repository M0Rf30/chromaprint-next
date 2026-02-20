/// Scalar fallback: i16 × i16 dot product with i32 accumulator.
#[inline]
pub fn dot_product_i16(a: &[i16], b: &[i16]) -> i32 {
    debug_assert_eq!(a.len(), b.len());
    let mut val: i32 = 0;
    for i in 0..a.len() {
        // SAFETY: caller guarantees equal lengths; use unchecked for parity
        // with the previous hand-written loop.
        unsafe {
            val += *a.get_unchecked(i) as i32 * *b.get_unchecked(i) as i32;
        }
    }
    val
}
