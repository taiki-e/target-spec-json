// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::mem;

use target_spec_json::*;

#[test]
#[cfg_attr(any(not(target_pointer_width = "64"), miri), ignore)] // we set -Z randomize-layout for Miri
fn size() {
    assert_eq!(mem::size_of::<Error>(), 40);
    assert_eq!(mem::size_of::<Arch>(), 1);
    assert_eq!(mem::size_of::<Os>(), 1);
    assert_eq!(mem::size_of::<Env>(), 1);
    assert_eq!(mem::size_of::<TargetEndian>(), 1);
    assert_eq!(mem::size_of::<PanicStrategy>(), 1);
    assert_eq!(mem::size_of::<AllTargetSpecs>(), 24);
    assert_eq!(mem::size_of::<TargetSpec>(), 1568);
    assert_eq!(mem::size_of::<Metadata>(), 48);
    assert_eq!(mem::size_of::<StackProbes>(), 40);
}