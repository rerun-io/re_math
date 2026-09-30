# `re_math`

[<img alt="github" src="https://img.shields.io/badge/github-rerun_io/re_math-8da0cb?logo=github" height="20">](https://github.com/rerun-io/re_math)
[![Latest version](https://img.shields.io/crates/v/re_math.svg)](https://crates.io/crates/re_math)
[![Documentation](https://docs.rs/re_math/badge.svg)](https://docs.rs/re_math)
[![Build Status](https://github.com/rerun-io/re_math/workflows/Rust/badge.svg)](https://github.com/rerun-io/re_math/actions?workflow=Rust)
![MIT](https://img.shields.io/badge/license-MIT-blue.svg)
![Apache](https://img.shields.io/badge/license-Apache-blue.svg)

`re_math` is a fork of [`macaw`](https://github.com/EmbarkStudios/macaw), maintained by [Rerun](https://github.com/rerun-io/rerun).
It keeps up with new [`glam`](https://github.com/bitshifter/glam-rs) releases and adds features the Rerun viewer needs.

## Feature flags
* `std` (default) — use the standard library.
* `libm` — use `libm` for math functions, for `no_std` targets.
* `serde` — `Serialize` and `Deserialize` for all types.
* `speedy` — `speedy` `Readable` and `Writable` for all types.
* `bytemuck` — `Pod` and `Zeroable` for `ColorRgba8`, and `bytemuck` support in `glam`.
* `mint` — `mint` conversions in `glam`.
* `debug_assert` / `assert` — `glam`'s extra assertions, in debug builds or always.

## MSRV
The minimum supported Rust version is 1.95.

## Versioning
`re_math` 0.33 is built on `glam` 0.33, continuing `macaw`'s numbering (`macaw` 0.30 is built on `glam` 0.30).
Later releases follow semver.
