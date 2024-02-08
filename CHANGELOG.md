# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org).

<!--
Note: In this file, do not use the hard wrap in the middle of a sentence for compatibility with GitHub comment style markdown rendering.
-->

## [Unreleased]

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

[Unreleased]: https://github.com/taiki-e/target-spec-json/compare/v0.1.9...HEAD
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
