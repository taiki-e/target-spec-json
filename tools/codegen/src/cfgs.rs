// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::io;

use duct::cmd;

use super::*;

pub(crate) fn gen() {
    gen_target_cfg();
}

// creates a full list of target cfg
fn gen_target_cfg() {
    let mut cfgs = vec![];
    for triple in target_list() {
        cfgs.append(&mut format!("{triple}:\n").into_bytes());
        let cfg_list = cfg_list(&triple);
        cfgs.extend(cfg_list.replace("debug_assertions\n", "").replace("\\\\", "\\").into_bytes());
        cfgs.push(b'\n');
        cfgs.push(b'\n');
    }
    write(workspace_root().join("tools/cfg"), &cfgs).unwrap();
}

/// Return a list of all built-in targets.
fn target_list() -> Vec<String> {
    cmd!("rustc", "--print", "target-list")
        .read()
        .unwrap()
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
        .collect()
}

fn cfg_list(target: &str) -> String {
    cmd!("rustc", "--print", "cfg", "--target", &target).read().unwrap()
}

fn write(path: impl AsRef<Path>, out: &[u8]) -> io::Result<()> {
    let path = path.as_ref();
    if path.is_file() && fs::read(path)? == out {
        return Ok(());
    }
    fs::write(path, out)?;
    Ok(())
}
