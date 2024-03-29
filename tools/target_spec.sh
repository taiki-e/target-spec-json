#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0 OR MIT
# shellcheck disable=SC2207
set -eEuo pipefail
IFS=$'\n\t'
cd "$(dirname "$0")"/..

# shellcheck disable=SC2154
trap 's=$?; echo >&2 "$0: error on line "${LINENO}": ${BASH_COMMAND}"; exit ${s}' ERR

# Generates code based on target-spec.
#
# USAGE:
#    ./tools/target_spec.sh
#
# This script is intended to be called by gen.sh, but can be called separately.

file="src/gen/target_spec.rs"
mkdir -p "$(dirname "${file}")"

target_arch=(
    # Architectures that do not included in builtin targets.
    # See also https://github.com/rust-lang/rust/blob/1.70.0/compiler/rustc_target/src/abi/call/mod.rs#L663
    # and https://github.com/rust-lang/rust/blob/1.70.0/src/bootstrap/lib.rs#L134.
    amdgpu
    asmjs
    nvptx
    spirv
    xtensa
)
target_os=(
    # Operating systems that do not included in builtin targets.
    zephyr # https://github.com/rust-lang/compiler-team/issues/629
)
target_env=(
    # Environments that do not included in builtin targets.
    # See also https://github.com/rust-lang/rust/blob/1.70.0/src/bootstrap/lib.rs#L131.
    libnx
    preview2 # https://github.com/rust-lang/rust/pull/119616
    # Used in the old rustc before https://github.com/rust-lang/rust/pull/119590
    eabihf
    gnueabihf
)
rustc -Z unstable-options --print all-target-specs-json >tools/target-spec.json
for target_spec in $(jq <tools/target-spec.json -c '. | to_entries | .[].value'); do
    target_arch+=("$(jq <<<"${target_spec}" -r '.arch')")
    os=$(jq <<<"${target_spec}" -r '.os')
    if [[ "${os}" == "null" ]]; then
        os=none
    fi
    target_os+=("${os}")
    env=$(jq <<<"${target_spec}" -r '.env')
    if [[ "${env}" == "null" ]]; then
        env=none
    fi
    target_env+=("${env}")
done

enum() {
    local name="$1"
    shift
    local variants=("$@")
    # sort and dedup
    IFS=$'\n'
    variants=($(LC_ALL=C sort -u <<<"${variants[*]}"))
    IFS=$'\n\t'
    if [[ -z ${EXHAUSTIVE:-} ]]; then
        local non_exhaustive='#[non_exhaustive]'
    else
        local non_exhaustive='#[allow(clippy::exhaustive_enums)]'
    fi
    if [[ -n "${DEFAULT_EMPTY_STR:-}" ]] && [[ -z "${DEFAULT:-}" ]]; then
        echo >&2 "warning: DEFAULT_EMPTY_STR requires DEFAULT"
    fi
    cat <<EOF
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
${non_exhaustive}
pub enum ${name} {
$(sed <<<"${variants[*]}" -E 's/^/    /g; s/$/,/g')
}
impl ${name} {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
EOF
    for v in "${variants[@]}"; do
        if [[ -n "${DEFAULT:-}" ]] && [[ -n "${DEFAULT_EMPTY_STR:-}" ]] && [[ "${v}" == "${DEFAULT}" ]]; then
            echo "            Self::${v} => \"\","
        else
            echo "            Self::${v} => \"${v}\","
        fi
    done
    cat <<EOF
        }
    }
}
EOF
    if [[ -n "${DEFAULT:-}" ]]; then
        # take &self for skip_serializing_if
        cat <<EOF
impl ${name} {
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub(crate) fn is_${DEFAULT}(&self) -> bool {
        matches!(self, Self::${DEFAULT})
    }
}
impl Default for ${name} {
    fn default() -> Self {
        Self::${DEFAULT}
    }
}
EOF
    fi
    cat <<EOF
impl core::fmt::Display for ${name} {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}
EOF
}
unset EXHAUSTIVE
unset DEFAULT
unset DEFAULT_EMPTY_STR

cat >"${file}" <<EOF
// SPDX-License-Identifier: Apache-2.0 OR MIT
// This file is @generated by $(basename "$0").
// It is not intended for manual editing.

#![allow(non_camel_case_types)]

use serde_derive::{Deserialize, Serialize};

$(enum Arch "${target_arch[@]}")

$(DEFAULT=none enum Os "${target_os[@]}")

$(DEFAULT_EMPTY_STR=1 DEFAULT=none enum Env "${target_env[@]}")

$(EXHAUSTIVE=1 DEFAULT=little enum TargetEndian big little)

$(EXHAUSTIVE=1 DEFAULT=unwind enum PanicStrategy abort unwind)
EOF
