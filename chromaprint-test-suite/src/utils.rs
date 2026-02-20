use std::fs;
use std::path::PathBuf;

/// Load a raw 16-bit little-endian PCM audio file from the C test data directory.
pub fn load_raw_audio(filename: &str) -> Vec<i16> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("..");
    path.push("chromaprint");
    path.push("tests");
    path.push("data");
    path.push(filename);

    let bytes = fs::read(&path).unwrap_or_else(|e| panic!("Failed to read {}: {}", path.display(), e));
    assert!(
        bytes.len() % 2 == 0,
        "Audio file has odd number of bytes: {}",
        bytes.len()
    );

    bytes
        .chunks_exact(2)
        .map(|chunk| i16::from_le_bytes([chunk[0], chunk[1]]))
        .collect()
}

/// Compare two raw fingerprint arrays with detailed diff output on failure.
pub fn compare_fingerprints(name: &str, c_raw: &[u32], rs_raw: &[u32]) {
    if c_raw == rs_raw {
        return;
    }

    let mut diffs = Vec::new();
    let max_len = c_raw.len().max(rs_raw.len());
    for i in 0..max_len {
        let c_val = c_raw.get(i);
        let rs_val = rs_raw.get(i);
        if c_val != rs_val {
            diffs.push(format!(
                "  [{}] C={} Rust={}",
                i,
                c_val.map_or("MISSING".to_string(), |v| v.to_string()),
                rs_val.map_or("MISSING".to_string(), |v| v.to_string()),
            ));
        }
        if diffs.len() >= 20 {
            diffs.push(format!("  ... and more (showing first 20 of {} entries)", max_len));
            break;
        }
    }

    panic!(
        "{}: fingerprints differ!\n  C length: {}, Rust length: {}\n  Differences:\n{}",
        name,
        c_raw.len(),
        rs_raw.len(),
        diffs.join("\n"),
    );
}
