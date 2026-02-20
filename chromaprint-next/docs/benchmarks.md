# Benchmarks

Performance comparison of the three chromaprint implementations using the Test2 (default) algorithm on mono 44100 Hz input.

Benchmarks are run with [Criterion.rs](https://github.com/bheisler/criterion.rs) via the `chromaprint-benchmarks` crate. The 2-second data point uses real audio from the test suite; longer durations use synthetic multi-frequency audio that exercises the full pipeline (resampling, FFT, chroma extraction, classification).

## Results

Measured on Apple M1 Pro, macOS, Rust 1.93.0 (release profile). The C reference library uses Accelerate/vDSP for FFT.

| Duration | C reference | chromaprint-next | rusty-chromaprint |
|----------|------------|-----------------|-------------------|
| 2 sec | 1.50 ms | 1.25 ms | 1.35 ms |
| 10 sec | 2.46 ms | 2.23 ms | 3.32 ms |
| 30 sec | 5.88 ms | 5.42 ms | 9.94 ms |
| 60 sec | 10.72 ms | 10.16 ms | 20.01 ms |
| 120 sec | 20.49 ms | 19.66 ms | 39.59 ms |

### Throughput (samples/sec)

| Duration | C reference | chromaprint-next | rusty-chromaprint |
|----------|------------|-----------------|-------------------|
| 2 sec | 118 Melem/s | 141 Melem/s | 131 Melem/s |
| 10 sec | 179 Melem/s | 198 Melem/s | 133 Melem/s |
| 30 sec | 225 Melem/s | 244 Melem/s | 133 Melem/s |
| 60 sec | 247 Melem/s | 260 Melem/s | 132 Melem/s |
| 120 sec | 258 Melem/s | 269 Melem/s | 134 Melem/s |

### Relative performance (vs C reference, 120-sec data point)

| Implementation | Time | Ratio |
|---------------|------|-------|
| **chromaprint-next** | **19.66 ms** | **0.96x (4% faster)** |
| C reference | 20.49 ms | 1.00x |
| rusty-chromaprint | 39.59 ms | 1.93x |

## Pipeline stage breakdown

Per-stage benchmarks for chromaprint-next (30 seconds of audio at 44100 Hz):

| Stage | Time | % of total |
|-------|------|-----------|
| Audio processing (resampling 44100→11025 Hz) | 2.68 ms | 51% |
| FFT (4096-point windowed real FFT + power spectrum) | 1.48 ms | 28% |
| Chroma extraction (spectrum → 12-band features) | 1.07 ms | 20% |
| Filter + normalize + classify | 0.06 ms | 1% |
| **Full pipeline** | **5.32 ms** | |

## Optimisations applied

chromaprint-next achieves its performance through several targeted optimisations in the hot paths, all of which preserve bit-identical output with the C reference:

1. **NEON SIMD for the resampler** — The 80-tap polyphase FIR filter's inner convolution loop uses `vmlal_s16` (signed multiply-accumulate long) to process 4 i16 pairs per instruction with two accumulator chains for instruction-level parallelism. On x86_64, equivalent SSE2 intrinsics (`_mm_madd_epi16`) are used. A scalar fallback is provided for other architectures. Integer SIMD produces mathematically identical results to scalar arithmetic — no floating-point rounding concerns.

2. **Bounds-check elimination in the resampler** — The hot-path convolution loop uses `get_unchecked` where the surrounding guard already proves the access is in-bounds, eliminating redundant bounds checks.

3. **Power-of-2 ring buffer in the FFT processor** — The sample ring buffer uses a bitmask instead of modulo for index wrapping, with bulk `copy_from_slice` for input and a fast path for non-wrapping frames.

4. **Inlining of chroma extraction** — `#[inline]` on the `ChromaExtractor::extract()` method allows the compiler to optimise across the call boundary.

## Observations

- **chromaprint-next is faster than the C reference** across all input sizes, despite the C library benefiting from Apple's Accelerate/vDSP for FFT. The NEON-optimised resampler and tight scalar code generation more than compensate.

- **chromaprint-next is ~2x faster** than rusty-chromaprint. The difference comes primarily from the resampler: chromaprint-next ports the lightweight fixed-point `av_resample` from FFmpeg (with SIMD acceleration), while rusty-chromaprint uses `rubato`, a more general-purpose resampler with higher per-sample overhead.

- **All implementations scale linearly** with input length, as expected for a streaming pipeline.

- **Resampling dominates** the pipeline at 51% of total time. FFT (28%) and chroma extraction (20%) are the next largest contributors. The filter/classify stage is negligible.

## Running the benchmarks

```
cargo bench -p chromaprint-benchmarks
```

Criterion generates HTML reports with violin plots and detailed statistics at `target/criterion/fingerprint_test2/report/index.html`.

Per-stage pipeline breakdown:

```
cargo bench -p chromaprint-benchmarks --bench stages
```

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
