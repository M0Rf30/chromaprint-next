# chromaprint-next

A pure-Rust, drop-in replacement for [Chromaprint](https://github.com/acoustid/chromaprint), the audio fingerprinting library used by [AcoustID](https://acoustid.org).

Produces **bit-identical fingerprints** to the C reference implementation across all five algorithm variants, [verified](chromaprint-next/docs/implementation.md#how-each-stage-matches-the-reference) by running both implementations side-by-side on identical inputs and asserting equality of raw fingerprints, encoded strings, and hashes.

**Faster than C** — chromaprint-next outperforms the C reference library by ~4% (269 vs 258 Melem/s at 120s), even though the C version uses Apple's hardware-optimised Accelerate/vDSP for FFT. This is achieved through NEON SIMD intrinsics in the resampler, bounds-check elimination, and careful memory layout. See [benchmarks](chromaprint-next/docs/benchmarks.md) for details.

## Repository structure

```
chromaprint/              C reference implementation (git submodule)
chromaprint-next/         Rust library crate
chromaprint-test-suite/   Cross-implementation comparison tests
chromaprint-benchmarks/   Criterion benchmarks comparing all three implementations
rusty-chromaprint/        Alternative Rust port by darksv (git submodule)
```

## Why another Rust port?

[rusty-chromaprint](https://github.com/darksv/rusty-chromaprint) is an existing pure-Rust port that works well for the default algorithm. However, it uses a different resampler, doesn't reproduce certain C bugs needed for database compatibility, and has incomplete presets for 3 of the 5 algorithm variants. chromaprint-next aims for strict bit-identical output across all configurations. See the [detailed comparison](chromaprint-next/docs/comparison-with-rusty-chromaprint.md).

## Getting started

```
git clone --recursive <repo-url>
cargo build -p chromaprint-next
```

## Running tests

The test suite compares both Rust implementations (chromaprint-next and rusty-chromaprint) against the C reference library, which is built automatically via CMake. It feeds identical PCM audio through all three implementations and asserts equality of raw fingerprints, compressed encodings, and hashes.

```
cargo test -p chromaprint-test-suite
```

Some rusty-chromaprint tests are `#[ignore]`d due to known incompatibilities (resampler differences, incomplete presets). To run them:

```
cargo test -p chromaprint-test-suite -- --ignored
```

## Benchmarks

The `chromaprint-benchmarks` crate compares throughput of all three implementations using the Test2 (default) algorithm at various audio durations.

```
cargo bench -p chromaprint-benchmarks
```

See [chromaprint-next/docs/benchmarks.md](chromaprint-next/docs/benchmarks.md) for results and analysis.

## Documentation

- [Implementation details](chromaprint-next/docs/implementation.md) — how the Rust port matches the C reference, performance characteristics, and known bug compatibility
- [Benchmarks](chromaprint-next/docs/benchmarks.md) — performance comparison across all three implementations
- [Comparison with rusty-chromaprint](chromaprint-next/docs/comparison-with-rusty-chromaprint.md) — detailed compatibility analysis

## License

Most of this project is licensed under the [MIT License](LICENSE-MIT). The resampler module (`chromaprint-next/src/audio/resample.rs`) is a port of FFmpeg's `av_resample` and is licensed under [LGPL-2.1-or-later](LICENSE-LGPL-2.1).

The combined work is therefore licensed as `MIT AND LGPL-2.1-or-later`.
