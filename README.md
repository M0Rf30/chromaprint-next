# chromaprint-next

A pure-Rust, drop-in replacement for [Chromaprint](https://github.com/acoustid/chromaprint), the audio fingerprinting library used by [AcoustID](https://acoustid.org).

Produces **bit-identical fingerprints** to the C reference implementation across all five algorithm variants, [verified](chromaprint-next/docs/implementation.md#how-each-stage-matches-the-reference) by running both implementations side-by-side on identical inputs and asserting equality of raw fingerprints, encoded strings, and hashes.

## Repository structure

```
chromaprint/              C reference implementation (git submodule)
chromaprint-next/         Rust library crate
chromaprint-test-suite/   Cross-implementation comparison tests
```

## Getting started

```
git clone --recursive <repo-url>
cargo build -p chromaprint-next
```

## Running tests

The test suite links both the C library (built automatically via CMake) and the Rust crate, feeding identical audio through both and asserting equality of raw fingerprints, encoded strings, and hashes.

```
cargo test -p chromaprint-test-suite
```

## Documentation

See [chromaprint-next/docs/implementation.md](chromaprint-next/docs/implementation.md) for details on how the Rust port matches the C reference, performance characteristics, and known bug compatibility.

## License

Most of this project is licensed under the [MIT License](LICENSE-MIT). The resampler module (`chromaprint-next/src/audio/resample.rs`) is a port of FFmpeg's `av_resample` and is licensed under [LGPL-2.1-or-later](LICENSE-LGPL-2.1).

The combined work is therefore licensed as `MIT AND LGPL-2.1-or-later`.
