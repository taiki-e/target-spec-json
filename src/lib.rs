// SPDX-License-Identifier: Apache-2.0 OR MIT

/*!
<!-- Note: Document from sync-markdown-to-rustdoc:start through sync-markdown-to-rustdoc:end
     is synchronized from README.md. Any changes to that range are not preserved. -->
<!-- tidy:sync-markdown-to-rustdoc:start -->

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

<!-- tidy:sync-markdown-to-rustdoc:end -->
*/

#![no_std]
#![doc(test(
    no_crate_inject,
    attr(allow(
        dead_code,
        unused_variables,
        clippy::undocumented_unsafe_blocks,
        clippy::unused_trait_names,
    ))
))]
#![forbid(unsafe_code)]
#![warn(
    // Lints that may help when writing public library.
    missing_debug_implementations,
    // missing_docs,
    clippy::alloc_instead_of_core,
    clippy::exhaustive_enums,
    clippy::exhaustive_structs,
    clippy::impl_trait_in_params,
    // clippy::missing_inline_in_public_items,
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
)]

extern crate alloc;
extern crate std;

#[cfg(test)]
#[path = "gen/tests/assert_impl.rs"]
mod assert_impl;
#[cfg(test)]
#[path = "gen/tests/track_size.rs"]
mod track_size;

#[path = "gen/target_spec.rs"]
mod target_spec;
pub use self::target_spec::{
    Arch, BinaryFormat, Env, Os, PanicStrategy, Sanitizer, TargetEndian, TargetFamily,
};

#[macro_use]
mod process;

mod error;

use alloc::{collections::BTreeMap, string::String, vec::Vec};
use core::ops;
use std::process::Command;

use serde_derive::{Deserialize, Serialize};

pub use self::error::Error;
use self::{error::Result, process::ProcessBuilder};

pub type AllTargetSpecs = BTreeMap<String, TargetSpec>;

// Refs:
// - https://github.com/rust-lang/rust/blob/HEAD/compiler/rustc_target/src/spec/mod.rs
// - https://github.com/rust-lang/rust/blob/c0bb3b98bb7aac24a37635e5d36d961e0b14f435/compiler/rustc_target/src/spec/json.rs
// TODO: use https://github.com/rust-lang/rust/pull/144498

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct TargetSpec {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abi_return_struct_as_int: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abi: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_weak_linkage: Option<bool>,
    pub arch: Arch,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asm_args: Option<Vec<String>>,
    #[serde(default = "default_true", skip_serializing_if = "Clone::clone")]
    pub atomic_cas: bool,
    #[serde(default, skip_serializing_if = "BinaryFormat::is_elf")]
    pub binary_format: BinaryFormat,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitcode_llvm_cmdline: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_enum_min_bits: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crt_objects_fallback: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crt_static_allows_dylibs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crt_static_default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crt_static_respected: Option<bool>,
    pub data_layout: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debuginfo_kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_adjusted_cabi: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_codegen_units: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_dwarf_version: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_hidden_visibility: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_sanitizers: Option<Vec<Sanitizer>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_uwtable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_access_external_data: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_redzone: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dll_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dll_suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dll_tls_export: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_linking: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eh_frame_header: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emit_debug_gdb_scripts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_abi: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_name: Option<String>,
    #[serde(default, skip_serializing_if = "Env::is_none")]
    pub env: Env,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executables: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exe_suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_emulated_tls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forces_embed_bitcode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_pointer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_sections: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_arange_section: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_rpath: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_thread_local: Option<bool>,
    #[serde(default, skip_serializing_if = "ops::Not::not")]
    pub has_thumb_interworking: bool,
    #[serde(default, skip_serializing_if = "ops::Not::not")]
    pub is_builtin: bool,
    #[serde(default, skip_serializing_if = "ops::Not::not")]
    pub is_like_android: bool,
    #[serde(default, skip_serializing_if = "ops::Not::not")]
    pub is_like_aix: bool,
    #[serde(default, skip_serializing_if = "ops::Not::not")]
    pub is_like_darwin: bool,
    #[serde(default, skip_serializing_if = "ops::Not::not")]
    pub is_like_gpu: bool,
    /// replaced by `is_like_darwin`
    #[serde(default, skip_serializing_if = "ops::Not::not")]
    pub is_like_osx: bool,
    #[serde(default, skip_serializing_if = "ops::Not::not")]
    pub is_like_solaris: bool,
    #[serde(default, skip_serializing_if = "ops::Not::not")]
    pub is_like_msvc: bool,
    #[serde(default, skip_serializing_if = "ops::Not::not")]
    pub is_like_vexos: bool,
    #[serde(default, skip_serializing_if = "ops::Not::not")]
    pub is_like_wasm: bool,
    #[serde(default, skip_serializing_if = "ops::Not::not")]
    pub is_like_windows: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub late_link_args: Option<BTreeMap<String, Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub late_link_args_dynamic: Option<BTreeMap<String, Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub late_link_args_static: Option<BTreeMap<String, Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_rdylib_exports: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_env: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_env_remove: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_self_contained: Option<BTreeMap<String, Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_script: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linker_flavor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linker_is_gnu: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lld_flavor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub llvm_abiname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub llvm_args: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub llvm_floatabi: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub llvm_mcount_intrinsic: Option<String>,
    pub llvm_target: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_needs_argc_argv: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_atomic_width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_functions: Option<String>,
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_atomic_width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_global_align: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_explicit_cpu: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs_plt: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_builtins: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_default_libraries: Option<bool>,
    #[serde(default, skip_serializing_if = "Os::is_none")]
    pub os: Os,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obj_is_bitcode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_cdylib: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_export_symbols: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "PanicStrategy::is_unwind")]
    pub panic_strategy: PanicStrategy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plt_by_default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_independent_executables: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_link_args: Option<BTreeMap<String, Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_link_objects: Option<BTreeMap<String, Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_link_objects_fallback: Option<BTreeMap<String, Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_link_args: Option<BTreeMap<String, Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_link_objects: Option<BTreeMap<String, Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_link_objects_fallback: Option<BTreeMap<String, Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relax_elf_relocations: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relocation_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relro_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_lto: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_uwtable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rustc_abi: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_debuginfo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_probes: Option<StackProbes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_initializer_must_be_acyclic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_position_independent_executables: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staticlib_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staticlib_suffix: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supported_sanitizers: Vec<Sanitizer>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supported_split_debuginfo: Vec<String>,
    #[serde(default = "default_true", skip_serializing_if = "Clone::clone")]
    pub supports_stack_protector: bool,
    #[serde(default, skip_serializing_if = "ops::Not::not")]
    pub supports_xray: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simd_types_indirect: Option<bool>,
    #[serde(default, skip_serializing_if = "ops::Not::not")]
    pub singlethread: bool,
    #[serde(default, skip_serializing_if = "TargetEndian::is_little")]
    pub target_endian: TargetEndian,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target_family: Vec<TargetFamily>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_mcount: Option<String>,
    // Integer since 1.89
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_c_int_width: Option<u32>,
    // Integer since 1.91
    pub target_pointer_width: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trap_unreachable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_ctors_section: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Metadata {
    pub description: Option<String>,
    pub host_tools: Option<bool>,
    pub std: Option<bool>,
    pub tier: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct StackProbes {
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_llvm_version_for_inline: Option<(u32, u32, u32)>,
}

fn default_true() -> bool {
    true
}

/// `<rustc> -Z unstable-options --print target-spec-json --target <target>`
pub fn target_spec_json(rustc: Command, target: &str) -> Result<TargetSpec> {
    let raw = ProcessBuilder::from_std(rustc)
        .args(["-Z", "unstable-options", "--print", "target-spec-json", "--target", target])
        .read()?;
    serde_json::from_str(&raw).map_err(Error::new)
}

/// `<rustc> -Z unstable-options --print all-target-specs-json`
pub fn all_target_specs_json(rustc: Command) -> Result<AllTargetSpecs> {
    let raw = ProcessBuilder::from_std(rustc)
        .args(["-Z", "unstable-options", "--print", "all-target-specs-json"])
        .read()?;
    serde_json::from_str(&raw).map_err(Error::new)
}

#[cfg(test)]
mod tests {
    use std::eprintln;

    use super::*;

    fn target_spec_json(target: &str) -> Result<(TargetSpec, String)> {
        let mut cmd = cmd!(
            "rustc",
            "-Z",
            "unstable-options",
            "--print",
            "target-spec-json",
            "--target",
            target
        );
        if !rustversion::cfg!(nightly) {
            cmd.env("RUSTC_BOOTSTRAP", "1");
        }
        let raw = cmd.read()?;
        Ok((serde_json::from_str(&raw).map_err(Error::new)?, raw))
    }

    fn all_target_specs_json() -> Result<(AllTargetSpecs, String)> {
        let mut cmd = cmd!("rustc", "-Z", "unstable-options", "--print", "all-target-specs-json");
        if !rustversion::cfg!(nightly) {
            cmd.env("RUSTC_BOOTSTRAP", "1");
        }
        let raw = cmd.read()?;
        Ok((serde_json::from_str(&raw).map_err(Error::new)?, raw))
    }

    // Skip pre-1.91 because target-pointer-width change
    #[rustversion::attr(before(1.91), ignore)]
    #[test]
    #[cfg_attr(miri, ignore)] // Miri doesn't support pipe2 (inside std::process::Command)
    fn parse_target_spec_json() {
        // builtin targets
        for target in cmd!("rustc", "--print", "target-list").read().unwrap().lines() {
            eprintln!("target={target}:");
            let (parsed, raw) = target_spec_json(target).unwrap();
            let deserialized = serde_json::to_string(&parsed).unwrap();
            assert_eq!(
                serde_json::from_str::<serde_json::Value>(&raw).unwrap(),
                serde_json::from_str::<serde_json::Value>(&deserialized).unwrap()
            );
        }
        eprintln!("all-targets:");
        let (parsed, raw) = all_target_specs_json().unwrap();
        let deserialized = serde_json::to_string(&parsed).unwrap();
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&raw).unwrap(),
            serde_json::from_str::<serde_json::Value>(&deserialized).unwrap()
        );
        // TODO: custom targets
        // for spec_path in fs::read_dir(fixtures_path().join("target-specs"))
        //     .unwrap()
        //     .map(|e| e.unwrap().path())
        // {
        // }
    }
}
