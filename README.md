# target-spec-json

[![crates.io](https://img.shields.io/crates/v/target-spec-json?style=flat-square&logo=rust)](https://crates.io/crates/target-spec-json)
[![docs.rs](https://img.shields.io/badge/docs.rs-target--spec--json-blue?style=flat-square&logo=docs.rs)](https://docs.rs/target-spec-json)
[![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue?style=flat-square)](#license)
[![msrv](https://img.shields.io/badge/msrv-1.71-blue?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![github actions](https://img.shields.io/github/actions/workflow/status/taiki-e/target-spec-json/ci.yml?branch=main&style=flat-square&logo=github)](https://github.com/taiki-e/target-spec-json/actions)

<!-- tidy:sync-markdown-to-rustdoc:start:src/lib.rs -->

Structured access to rustc `--print target-spec-json` and `--print all-target-specs-json`.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
target-spec-json = "0.2"
```

## Compatibility

Both `--print target-spec-json` and `--print all-target-specs-json` are unstable interfaces and may not work with certain version combinations of Rust versions and `target-spec-json` versions.

The following combinations have been confirmed to work:

| target-spec-json | Rust                                    |
| ---------------- | --------------------------------------- |
| 0.2.7            | nightly-2026-01-09                      |
| 0.2.6            | nightly-2025-11-29 - nightly-2026-01-08 |
| 0.2.5            | nightly-2025-10-08 - nightly-2025-10-30 |
| 0.2.4            | nightly-2025-09-23 - nightly-2025-10-07 |
| 0.2.3            | nightly-2025-09-01 - nightly-2025-09-22 |
| 0.2.2            | nightly-2025-08-31                      |
| 0.2.1            | nightly-2025-08-10 - nightly-2025-08-30 |
| 0.2.0            | nightly-2025-07-06 - nightly-2025-08-08 |

## Related Projects

- [cargo-config2]: Library to load and resolve Cargo configuration.

[cargo-config2]: https://github.com/taiki-e/cargo-config2

<!-- tidy:sync-markdown-to-rustdoc:end -->

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
