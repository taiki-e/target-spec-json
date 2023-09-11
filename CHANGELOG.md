# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org).

<!--
Note: In this file, do not use the hard wrap in the middle of a sentence for compatibility with GitHub comment style markdown rendering.
-->

## [Unreleased]

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

[Unreleased]: https://github.com/taiki-e/target-spec-json/compare/v0.1.3...HEAD
[0.1.3]: https://github.com/taiki-e/target-spec-json/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/taiki-e/target-spec-json/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/taiki-e/target-spec-json/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/taiki-e/target-spec-json/releases/tag/v0.1.0
