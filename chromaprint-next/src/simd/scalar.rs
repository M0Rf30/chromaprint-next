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

/// Scalar fallback: mix interleaved stereo i16 pairs to mono by averaging.
/// `input` has `output.len() * 2` elements (L0,R0,L1,R1,...).
/// output[i] = (input[2*i] + input[2*i+1]) / 2
#[inline]
pub fn stereo_to_mono(input: &[i16], output: &mut [i16]) {
    debug_assert_eq!(input.len(), output.len() * 2);
    for i in 0..output.len() {
        output[i] = ((input[i * 2] as i32 + input[i * 2 + 1] as i32) / 2) as i16;
    }
}

/// Scalar fallback: convert i16 samples to f32 and multiply by window coefficients.
/// output[i] = input[i] as f32 * window[i]
#[inline]
pub fn window_apply(input: &[i16], window: &[f32], output: &mut [f32]) {
    debug_assert_eq!(input.len(), window.len());
    debug_assert_eq!(input.len(), output.len());
    for i in 0..input.len() {
        output[i] = input[i] as f32 * window[i];
    }
}
