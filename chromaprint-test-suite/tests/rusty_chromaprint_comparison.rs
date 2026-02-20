//! Compare rusty-chromaprint against the C reference implementation.
//!
//! rusty-chromaprint is an independent Rust port by darksv. These tests verify
//! that it produces identical raw fingerprints to the C library.

use chromaprint_test_suite::ffi::{CChromaprint, c_encode_fingerprint};
use chromaprint_test_suite::utils::{compare_fingerprints_labeled, load_raw_audio};
use rusty_chromaprint::{Configuration, FingerprintCompressor, Fingerprinter as RustyFingerprinter};

fn rusty_config(algo: i32) -> Configuration {
    match algo {
        0 => Configuration::preset_test1(),
        1 => Configuration::preset_test2(),
        2 => Configuration::preset_test3(),
        3 => Configuration::preset_test4(),
        4 => Configuration::preset_test5(),
        _ => panic!("Unknown algorithm: {}", algo),
    }
}

/// Generate fingerprints from both C and rusty-chromaprint, returning raw u32 arrays.
fn fingerprint_both(
    data: &[i16],
    sample_rate: i32,
    num_channels: i32,
    c_algo: i32,
) -> (Vec<u32>, Vec<u32>) {
    // C implementation
    let c_ctx = CChromaprint::new(c_algo);
    c_ctx.start(sample_rate, num_channels);
    c_ctx.feed(data);
    c_ctx.finish();
    let c_raw = c_ctx.get_raw_fingerprint();

    // rusty-chromaprint
    let config = rusty_config(c_algo);
    let mut rusty_ctx = RustyFingerprinter::new(&config);
    rusty_ctx
        .start(sample_rate as u32, num_channels as u32)
        .unwrap();
    rusty_ctx.consume(data);
    rusty_ctx.finish();
    let rusty_raw = rusty_ctx.fingerprint().to_vec();

    (c_raw, rusty_raw)
}

const ALGORITHM_TEST2: i32 = 1;

// ---------------------------------------------------------------------------
// Fingerprint comparison tests (Test2 algorithm, various audio inputs)
// ---------------------------------------------------------------------------

// This test loads a stereo file but declares 1 channel, so the interleaved stereo data is
// treated as a double-length mono stream. The larger sample count amplifies small numerical
// differences between rusty-chromaprint's rubato resampler and the C library's internal
// resampler, causing a few sub-fingerprint bits to differ.
#[test]
#[ignore = "resampler differences: rusty-chromaprint (rubato) vs C produce slightly different results on long mono streams"]
fn test_rusty_stereo_44100_as_mono() {
    let data = load_raw_audio("test_stereo_44100.raw");
    let (c_raw, rusty_raw) = fingerprint_both(&data, 44100, 1, ALGORITHM_TEST2);
    compare_fingerprints_labeled("stereo_44100_as_mono", "C", &c_raw, "rusty-chromaprint", &rusty_raw);
}

#[test]
fn test_rusty_silence() {
    let data = vec![0i16; 130 * 1024];
    let (c_raw, rusty_raw) = fingerprint_both(&data, 44100, 1, ALGORITHM_TEST2);
    compare_fingerprints_labeled("silence", "C", &c_raw, "rusty-chromaprint", &rusty_raw);
}

#[test]
fn test_rusty_mono_44100() {
    let data = load_raw_audio("test_mono_44100.raw");
    let (c_raw, rusty_raw) = fingerprint_both(&data, 44100, 1, ALGORITHM_TEST2);
    compare_fingerprints_labeled("mono_44100", "C", &c_raw, "rusty-chromaprint", &rusty_raw);
}

#[test]
fn test_rusty_mono_11025() {
    let data = load_raw_audio("test_mono_11025.raw");
    let (c_raw, rusty_raw) = fingerprint_both(&data, 11025, 1, ALGORITHM_TEST2);
    compare_fingerprints_labeled("mono_11025", "C", &c_raw, "rusty-chromaprint", &rusty_raw);
}

#[test]
fn test_rusty_stereo_44100() {
    let data = load_raw_audio("test_stereo_44100.raw");
    let (c_raw, rusty_raw) = fingerprint_both(&data, 44100, 2, ALGORITHM_TEST2);
    compare_fingerprints_labeled("stereo_44100", "C", &c_raw, "rusty-chromaprint", &rusty_raw);
}

#[test]
fn test_rusty_mono_8000() {
    let data = load_raw_audio("test_mono_8000.raw");
    let (c_raw, rusty_raw) = fingerprint_both(&data, 8000, 1, ALGORITHM_TEST2);
    compare_fingerprints_labeled("mono_8000", "C", &c_raw, "rusty-chromaprint", &rusty_raw);
}

// ---------------------------------------------------------------------------
// Algorithm comparison tests
// ---------------------------------------------------------------------------

#[test]
fn test_rusty_algorithm_test1() {
    let data = load_raw_audio("test_mono_44100.raw");
    let (c_raw, rusty_raw) = fingerprint_both(&data, 44100, 1, 0);
    compare_fingerprints_labeled("TEST1", "C", &c_raw, "rusty-chromaprint", &rusty_raw);
}

#[test]
fn test_rusty_algorithm_test2() {
    let data = load_raw_audio("test_mono_44100.raw");
    let (c_raw, rusty_raw) = fingerprint_both(&data, 44100, 1, 1);
    compare_fingerprints_labeled("TEST2", "C", &c_raw, "rusty-chromaprint", &rusty_raw);
}

// rusty-chromaprint's preset_test3 uses DEFAULT_FRAME_OVERLAP (2731), while the C library
// and chromaprint-next use frame_overlap=0 (a bug in the C implementation that was preserved
// for compatibility). This causes the fingerprints to differ.
#[test]
#[ignore = "rusty-chromaprint preset_test3 uses different frame_overlap than C (2731 vs 0)"]
fn test_rusty_algorithm_test3() {
    let data = load_raw_audio("test_mono_44100.raw");
    let (c_raw, rusty_raw) = fingerprint_both(&data, 44100, 1, 2);
    compare_fingerprints_labeled("TEST3", "C", &c_raw, "rusty-chromaprint", &rusty_raw);
}

// rusty-chromaprint's preset_test4 only sets silence removal but does not configure
// classifiers or filter coefficients, resulting in empty fingerprints.
#[test]
#[ignore = "rusty-chromaprint preset_test4 is incomplete (missing classifiers/coefficients)"]
fn test_rusty_algorithm_test4() {
    let data = load_raw_audio("test_mono_44100.raw");
    let (c_raw, rusty_raw) = fingerprint_both(&data, 44100, 1, 3);
    compare_fingerprints_labeled("TEST4", "C", &c_raw, "rusty-chromaprint", &rusty_raw);
}

// rusty-chromaprint's preset_test5 only sets frame size/overlap but does not configure
// classifiers or filter coefficients, resulting in empty fingerprints.
#[test]
#[ignore = "rusty-chromaprint preset_test5 is incomplete (missing classifiers/coefficients)"]
fn test_rusty_algorithm_test5() {
    let data = load_raw_audio("test_mono_44100.raw");
    let (c_raw, rusty_raw) = fingerprint_both(&data, 44100, 1, 4);
    compare_fingerprints_labeled("TEST5", "C", &c_raw, "rusty-chromaprint", &rusty_raw);
}

// ---------------------------------------------------------------------------
// Compression comparison
// ---------------------------------------------------------------------------

#[test]
fn test_rusty_compression() {
    let data = load_raw_audio("test_mono_44100.raw");
    let (c_raw, rusty_raw) = fingerprint_both(&data, 44100, 1, ALGORITHM_TEST2);

    // First confirm the raw fingerprints match (prerequisite for compression comparison)
    compare_fingerprints_labeled("compression_prereq", "C", &c_raw, "rusty-chromaprint", &rusty_raw);

    // Compare compressed binary output
    let c_compressed = c_encode_fingerprint(&c_raw, ALGORITHM_TEST2, false);
    let config = rusty_config(ALGORITHM_TEST2);
    let rusty_compressed = FingerprintCompressor::from(&config).compress(&rusty_raw);

    assert_eq!(
        c_compressed, rusty_compressed,
        "Compressed fingerprints differ:\n  C length: {}, rusty-chromaprint length: {}\n  C first 20 bytes: {:?}\n  rusty first 20 bytes: {:?}",
        c_compressed.len(),
        rusty_compressed.len(),
        &c_compressed[..c_compressed.len().min(20)],
        &rusty_compressed[..rusty_compressed.len().min(20)],
    );
}
