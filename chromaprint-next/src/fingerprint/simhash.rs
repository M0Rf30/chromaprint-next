/// Compute SimHash from a raw fingerprint array.
/// Produces a single 32-bit locality-sensitive hash.
pub fn simhash(data: &[u32]) -> u32 {
    if data.is_empty() {
        return 0;
    }

    let mut counts = [0u32; 32];
    for &word in data {
        for j in 0..32 {
            counts[j] += (word >> j) & 1;
        }
    }

    let threshold = (data.len() as u32) / 2;
    let mut hash: u32 = 0;
    for i in 0..32 {
        if counts[i] > threshold {
            hash |= 1 << i;
        }
    }

    hash
}
