use crate::error::{Error, Result};
use crate::types::Algorithm;

const MAX_NORMAL_VALUE: u8 = 7;

/// Decompress a base64-encoded fingerprint string into raw sub-fingerprints.
pub fn decompress(encoded: &str) -> Result<(Vec<u32>, Algorithm)> {
    let data = crate::codec::base64::decode(encoded)?;
    decompress_raw(&data)
}

/// Decompress raw binary fingerprint data into sub-fingerprints.
pub fn decompress_raw(input: &[u8]) -> Result<(Vec<u32>, Algorithm)> {
    if input.len() < 4 {
        return Err(Error::InvalidFingerprint);
    }

    let algorithm = Algorithm::from_u8(input[0]).ok_or(Error::UnknownAlgorithm(input[0]))?;
    let size = ((input[1] as usize) << 16)
             | ((input[2] as usize) << 8)
             | (input[3] as usize);

    if size == 0 {
        return Ok((Vec::new(), algorithm));
    }

    let offset = 4;
    let mut bits = unpack_int3_array(&input[offset..]);

    // Find the actual end and count exceptional bits
    let mut found_values = 0usize;
    let mut num_exceptional = 0usize;
    let mut actual_len = bits.len();
    for (i, &b) in bits.iter().enumerate() {
        if b == 0 {
            found_values += 1;
            if found_values == size {
                actual_len = i + 1;
                break;
            }
        } else if b == MAX_NORMAL_VALUE {
            num_exceptional += 1;
        }
    }

    if found_values != size {
        return Err(Error::InvalidFingerprint);
    }
    bits.truncate(actual_len);

    // Unpack exceptional bits
    let packed3_size = (actual_len * 3 + 7) / 8;
    let exceptional_offset = offset + packed3_size;
    if num_exceptional > 0 && exceptional_offset < input.len() {
        let exceptional = unpack_int5_array(&input[exceptional_offset..]);
        let mut j = 0;
        for b in bits.iter_mut() {
            if *b == MAX_NORMAL_VALUE {
                if j < exceptional.len() {
                    *b += exceptional[j];
                }
                j += 1;
            }
        }
    }

    // Reconstruct fingerprint from bit deltas
    let mut output = vec![0u32; size];
    let mut i = 0;
    let mut last_bit: u32 = 0;
    let mut value: u32 = 0;
    for &bit in &bits {
        if bit == 0 {
            output[i] = value;
            last_bit = 0;
            i += 1;
        } else {
            last_bit += bit as u32;
            value ^= 1 << (last_bit - 1);
        }
    }

    Ok((output, algorithm))
}

fn unpack_int3_array(input: &[u8]) -> Vec<u8> {
    let size = input.len();
    let output_size = size * 8 / 3;
    let mut out = Vec::with_capacity(output_size);
    let mut si = 0;

    while si + 3 <= size {
        let s0 = input[si];
        let s1 = input[si + 1];
        let s2 = input[si + 2];
        out.push(s0 & 0x07);
        out.push((s0 & 0x38) >> 3);
        out.push(((s0 & 0xc0) >> 6) | ((s1 & 0x01) << 2));
        out.push((s1 & 0x0e) >> 1);
        out.push((s1 & 0x70) >> 4);
        out.push(((s1 & 0x80) >> 7) | ((s2 & 0x03) << 1));
        out.push((s2 & 0x1c) >> 2);
        out.push((s2 & 0xe0) >> 5);
        si += 3;
    }

    let remaining = size - si;
    if remaining == 2 {
        let s0 = input[si];
        let s1 = input[si + 1];
        out.push(s0 & 0x07);
        out.push((s0 & 0x38) >> 3);
        out.push(((s0 & 0xc0) >> 6) | ((s1 & 0x01) << 2));
        out.push((s1 & 0x0e) >> 1);
        out.push((s1 & 0x70) >> 4);
    } else if remaining == 1 {
        let s0 = input[si];
        out.push(s0 & 0x07);
        out.push((s0 & 0x38) >> 3);
    }

    out
}

fn unpack_int5_array(input: &[u8]) -> Vec<u8> {
    let size = input.len();
    let output_size = size * 8 / 5;
    let mut out = Vec::with_capacity(output_size);
    let mut si = 0;

    while si + 5 <= size {
        let s0 = input[si];
        let s1 = input[si + 1];
        let s2 = input[si + 2];
        let s3 = input[si + 3];
        let s4 = input[si + 4];
        out.push(s0 & 0x1f);
        out.push(((s0 & 0xe0) >> 5) | ((s1 & 0x03) << 3));
        out.push((s1 & 0x7c) >> 2);
        out.push(((s1 & 0x80) >> 7) | ((s2 & 0x0f) << 1));
        out.push(((s2 & 0xf0) >> 4) | ((s3 & 0x01) << 4));
        out.push((s3 & 0x3e) >> 1);
        out.push(((s3 & 0xc0) >> 6) | ((s4 & 0x07) << 2));
        out.push((s4 & 0xf8) >> 3);
        si += 5;
    }

    let remaining = size - si;
    if remaining >= 1 {
        out.push(input[si] & 0x1f);
    }
    if remaining >= 2 {
        out.push(((input[si] & 0xe0) >> 5) | ((input[si + 1] & 0x03) << 3));
        out.push((input[si + 1] & 0x7c) >> 2);
    }
    if remaining >= 3 {
        out.push(((input[si + 1] & 0x80) >> 7) | ((input[si + 2] & 0x0f) << 1));
        out.push(((input[si + 2] & 0xf0) >> 4) | (if remaining >= 4 { (input[si + 3] & 0x01) << 4 } else { 0 }));
    }
    if remaining >= 4 {
        out.push((input[si + 3] & 0x3e) >> 1);
    }

    out
}
