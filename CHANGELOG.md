# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org).

Releases may yanked if there is a security bug, a soundness bug, or a regression.

<!--
Note: In this file, do not use the hard wrap in the middle of a sentence for compatibility with GitHub comment style markdown rendering.
-->

## [Unreleased]

## [0.1.22] - 2025-01-04

- Add `TargetSpec::llvm_floatabi`.

## [0.1.21] - 2024-11-02

- Add `Os::psx`.

## [0.1.20] - 2024-09-06

- Add `Os::rtems`.

## [0.1.19] - 2024-08-28

- Add `Os::trusty`.

## [0.1.18] - 2024-07-31

- Add `Os::nuttx`.

## [0.1.17] - 2024-06-14

- Add `Env::p1`.

## [0.1.16] - 2024-05-24

- Add `TargetSpec::link_self_contained`.

## [0.1.15] - 2024-04-07

- Add `Os::visionos`.

## [0.1.14] - 2024-03-12

- Add `TargetSpec::metadata`.

## [0.1.13] - 2024-03-09

- Add `TargetSpec::description`.

## [0.1.12] - 2024-03-08

- Add `Arch::arm64ec`.

## [0.1.11] - 2024-03-05

- Add `#[must_use]` to `as_str` methods.

- Add `Env::p2`.

## [0.1.10] - 2024-02-08

- Add `TargetSpec::direct_access_external_data`.

## [0.1.9] - 2024-01-25

- Add `Env::preview2`.

## [0.1.8] - 2024-01-24

- Add `Os::zkvm`.

## [0.1.7] - 2023-10-30

- Add `Os::zephyr`.

## [0.1.6] - 2023-10-18

- Improve compile time.

## [0.1.5] - 2023-09-25

- Add `TargetSpec::entry_abi`.

## [0.1.4] - 2023-09-11

- Remove dependency on `shell-escape`.

## [0.1.3] - 2023-08-26

- Rename `TargetArch` to `Arch`, `TargetEnv` to `Env`, `TargetOs` to `Os` for consistency with `TargetSpec` field names.

  The old names are kept and can be used as aliases.

- Add `Os::hurd`.

## [0.1.2] - 2023-08-19

- Add `TargetSpec::llvm_mcount_intrinsic`.

## [0.1.1] - 2023-07-18

- Add `TargetArch::{mips32r6,mips64r6}`.

## [0.1.0] - 2023-07-15

Initial release

[Unreleased]: https://github.com/taiki-e/target-spec-json/compare/v0.1.22...HEAD
[0.1.22]: https://github.com/taiki-e/target-spec-json/compare/v0.1.21...v0.1.22
[0.1.21]: https://github.com/taiki-e/target-spec-json/compare/v0.1.20...v0.1.21
[0.1.20]: https://github.com/taiki-e/target-spec-json/compare/v0.1.19...v0.1.20
[0.1.19]: https://github.com/taiki-e/target-spec-json/compare/v0.1.18...v0.1.19
[0.1.18]: https://github.com/taiki-e/target-spec-json/compare/v0.1.17...v0.1.18
[0.1.17]: https://github.com/taiki-e/target-spec-json/compare/v0.1.16...v0.1.17
[0.1.16]: https://github.com/taiki-e/target-spec-json/compare/v0.1.15...v0.1.16
[0.1.15]: https://github.com/taiki-e/target-spec-json/compare/v0.1.14...v0.1.15
[0.1.14]: https://github.com/taiki-e/target-spec-json/compare/v0.1.13...v0.1.14
[0.1.13]: https://github.com/taiki-e/target-spec-json/compare/v0.1.12...v0.1.13
[0.1.12]: https://github.com/taiki-e/target-spec-json/compare/v0.1.11...v0.1.12
[0.1.11]: https://github.com/taiki-e/target-spec-json/compare/v0.1.10...v0.1.11
[0.1.10]: https://github.com/taiki-e/target-spec-json/compare/v0.1.9...v0.1.10
[0.1.9]: https://github.com/taiki-e/target-spec-json/compare/v0.1.8...v0.1.9
[0.1.8]: https://github.com/taiki-e/target-spec-json/compare/v0.1.7...v0.1.8
[0.1.7]: https://github.com/taiki-e/target-spec-json/compare/v0.1.6...v0.1.7
[0.1.6]: https://github.com/taiki-e/target-spec-json/compare/v0.1.5...v0.1.6
[0.1.5]: https://github.com/taiki-e/target-spec-json/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/taiki-e/target-spec-json/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/taiki-e/target-spec-json/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/taiki-e/target-spec-json/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/taiki-e/target-spec-json/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/taiki-e/target-spec-json/releases/tag/v0.1.0
