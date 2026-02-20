use chromaprint_test_suite::ffi::CChromaprint;
use chromaprint_test_suite::utils::{compare_fingerprints, load_raw_audio};
use chromaprint::{Algorithm, Fingerprinter};

fn test_algorithm(c_algo: i32, rs_algo: Algorithm, name: &str) {
    let data = load_raw_audio("test_stereo_44100.raw");

    // C implementation
    let c_ctx = CChromaprint::new(c_algo);
    c_ctx.start(44100, 1);
    c_ctx.feed(&data);
    c_ctx.finish();
    let c_raw = c_ctx.get_raw_fingerprint();
    let c_encoded = c_ctx.get_fingerprint();
    let c_hash = c_ctx.get_fingerprint_hash();

    // Rust implementation
    let mut rs_ctx = Fingerprinter::new(rs_algo);
    rs_ctx.start(44100, 1).unwrap();
    rs_ctx.feed(&data).unwrap();
    rs_ctx.finish().unwrap();
    let rs_raw = rs_ctx.fingerprint().to_vec();
    let rs_encoded = rs_ctx.encode();
    let rs_hash = rs_ctx.hash();

    compare_fingerprints(name, &c_raw, &rs_raw);
    assert_eq!(
        c_encoded, rs_encoded,
        "{}: encoded fingerprints differ",
        name
    );
    assert_eq!(c_hash, rs_hash, "{}: hashes differ", name);
}

#[test]
fn test_algorithm_test1() {
    test_algorithm(0, Algorithm::Test1, "TEST1");
}

#[test]
fn test_algorithm_test2() {
    test_algorithm(1, Algorithm::Test2, "TEST2");
}

#[test]
fn test_algorithm_test3() {
    test_algorithm(2, Algorithm::Test3, "TEST3");
}

#[test]
fn test_algorithm_test4() {
    test_algorithm(3, Algorithm::Test4, "TEST4");
}

#[test]
fn test_algorithm_test5() {
    test_algorithm(4, Algorithm::Test5, "TEST5");
}
