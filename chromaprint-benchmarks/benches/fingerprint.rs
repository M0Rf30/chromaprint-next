use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

use chromaprint_benchmarks::ffi::CChromaprint;
use chromaprint_benchmarks::utils::{generate_synthetic_audio, load_raw_audio};

const ALGORITHM_TEST2: i32 = 1;

fn bench_c_reference(data: &[i16], sample_rate: i32, num_channels: i32) {
    let ctx = CChromaprint::new(ALGORITHM_TEST2);
    ctx.start(sample_rate, num_channels);
    ctx.feed(data);
    ctx.finish();
    let _raw = ctx.get_raw_fingerprint();
}

fn bench_chromaprint_next(data: &[i16], sample_rate: u32, num_channels: u16) {
    let mut fp = chromaprint::Fingerprinter::new(chromaprint::Algorithm::Test2);
    fp.start(sample_rate, num_channels).unwrap();
    fp.feed(data).unwrap();
    fp.finish().unwrap();
    let _raw = fp.fingerprint();
}

fn bench_rusty_chromaprint(data: &[i16], sample_rate: u32, num_channels: u32) {
    let config = rusty_chromaprint::Configuration::preset_test2();
    let mut fp = rusty_chromaprint::Fingerprinter::new(&config);
    fp.start(sample_rate, num_channels).unwrap();
    fp.consume(data);
    fp.finish();
    let _raw = fp.fingerprint();
}

fn fingerprint_benchmarks(c: &mut Criterion) {
    // Load real audio (stereo file treated as mono — same as the test suite)
    let audio_2sec = load_raw_audio("test_stereo_44100.raw");

    // Generate synthetic audio at various durations
    let audio_10sec = generate_synthetic_audio(10.0, 44100);
    let audio_30sec = generate_synthetic_audio(30.0, 44100);
    let audio_60sec = generate_synthetic_audio(60.0, 44100);
    let audio_120sec = generate_synthetic_audio(120.0, 44100);

    let datasets: Vec<(&str, &[i16])> = vec![
        ("2sec", &audio_2sec),
        ("10sec", &audio_10sec),
        ("30sec", &audio_30sec),
        ("60sec", &audio_60sec),
        ("120sec", &audio_120sec),
    ];

    let mut group = c.benchmark_group("fingerprint_test2");

    for (label, data) in &datasets {
        group.throughput(Throughput::Elements(data.len() as u64));

        group.bench_with_input(
            BenchmarkId::new("c_reference", label),
            data,
            |b, data| {
                b.iter(|| bench_c_reference(data, 44100, 1));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("chromaprint_next", label),
            data,
            |b, data| {
                b.iter(|| bench_chromaprint_next(data, 44100, 1));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("rusty_chromaprint", label),
            data,
            |b, data| {
                b.iter(|| bench_rusty_chromaprint(data, 44100, 1));
            },
        );
    }

    group.finish();
}

criterion_group!(benches, fingerprint_benchmarks);
criterion_main!(benches);
