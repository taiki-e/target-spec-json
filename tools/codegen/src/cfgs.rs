// SPDX-License-Identifier: Apache-2.0 OR MIT

use duct::cmd;

use super::*;

pub(crate) fn gen() -> Result<()> {
    gen_target_cfg()?;
    Ok(())
}

// creates a full list of target cfg
fn gen_target_cfg() -> Result<()> {
    let mut cfgs = vec![];
    for triple in target_list()? {
        cfgs.append(&mut format!("{triple}:\n").into_bytes());
        let cfg_list = cfg_list(&triple)?;
        cfgs.extend(cfg_list.replace("debug_assertions\n", "").replace("\\\\", "\\").into_bytes());
        cfgs.push(b'\n');
        cfgs.push(b'\n');
    }
    write(workspace_root().join("tools/cfg"), &cfgs)?;
    Ok(())
}

/// Return a list of all built-in targets.
fn target_list() -> Result<Vec<String>> {
    Ok(cmd!("rustc", "--print", "target-list")
        .read()?
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
        .collect())
}

fn cfg_list(target: &str) -> Result<String> {
    Ok(cmd!("rustc", "--print", "cfg", "--target", &target).read()?)
}

fn write(path: impl AsRef<Path>, out: &[u8]) -> Result<()> {
    let path = path.as_ref();
    if path.is_file() && fs::read(path)? == out {
        return Ok(());
    }
    fs::write(path, out)?;
    Ok(())
}
