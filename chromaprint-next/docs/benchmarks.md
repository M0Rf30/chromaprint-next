# Benchmarks

Performance comparison of the three chromaprint implementations using the Test2 (default) algorithm on mono 44100 Hz input.

Benchmarks are run with [Criterion.rs](https://github.com/bheisler/criterion.rs) via the `chromaprint-benchmarks` crate. The 2-second data point uses real audio from the test suite; longer durations use synthetic multi-frequency audio that exercises the full pipeline (resampling, FFT, chroma extraction, classification).

## Results

Measured on Apple M1 Pro, macOS, Rust 1.93.0 (release profile). The C reference library uses Accelerate/vDSP for FFT.

| Duration | C reference | chromaprint-next | rusty-chromaprint |
|----------|------------|-----------------|-------------------|
| 2 sec | 1.49 ms | 1.87 ms | 1.35 ms |
| 10 sec | 2.43 ms | 3.70 ms | 3.36 ms |
| 30 sec | 5.66 ms | 9.90 ms | 10.01 ms |
| 60 sec | 10.59 ms | 19.10 ms | 19.89 ms |
| 120 sec | 20.35 ms | 37.45 ms | 39.98 ms |

### Throughput (samples/sec)

| Duration | C reference | chromaprint-next | rusty-chromaprint |
|----------|------------|-----------------|-------------------|
| 2 sec | 118 Melem/s | 94 Melem/s | 131 Melem/s |
| 10 sec | 181 Melem/s | 119 Melem/s | 131 Melem/s |
| 30 sec | 234 Melem/s | 134 Melem/s | 132 Melem/s |
| 60 sec | 250 Melem/s | 139 Melem/s | 133 Melem/s |
| 120 sec | 260 Melem/s | 141 Melem/s | 132 Melem/s |

### Relative performance (vs C reference, 120-sec data point)

| Implementation | Time | Ratio |
|---------------|------|-------|
| C reference | 20.35 ms | 1.00x |
| chromaprint-next | 37.45 ms | 1.84x |
| rusty-chromaprint | 39.98 ms | 1.96x |

## Observations

- **C reference is ~1.8-2x faster** than both Rust implementations. On macOS this is largely attributable to the Accelerate/vDSP FFT, which is hardware-optimized for Apple Silicon. The Rust implementations use `realfft` and `rustfft` respectively, which are pure-Rust FFT libraries without platform-specific SIMD specialisation.

- **chromaprint-next is ~7% faster** than rusty-chromaprint at larger input sizes. The gap is modest and likely comes from differences in the resampler (chromaprint-next ports the lightweight fixed-point `av_resample` from FFmpeg, while rusty-chromaprint uses `rubato`, a more general-purpose resampler).

- **All implementations scale linearly** with input length, as expected for a streaming pipeline.

- **Both Rust implementations are fast enough for practical use** — fingerprinting 120 seconds of audio in ~37-40 ms means processing is roughly 3000x faster than real-time.

## Running the benchmarks

```
cargo bench -p chromaprint-benchmarks
```

Criterion generates HTML reports with violin plots and detailed statistics at `target/criterion/fingerprint_test2/report/index.html`.

To compare against a saved baseline:

```
cargo bench -p chromaprint-benchmarks -- --save-baseline v1
# ... make changes ...
cargo bench -p chromaprint-benchmarks -- --baseline v1
```

To run a specific data size:

```
cargo bench -p chromaprint-benchmarks -- "30sec"
```
