//! `update`: disabled in this fork.
//!
//! Upstream's `update` pulled the newest `vX.Y.Z` tag from the plugin's
//! `origin` and reinstalled it. Upstream's newest releases are Shepherdr, a
//! different product whose installer rewrites Herdr's config, removes skill
//! links and stops the ticker, and this fork carries upstream's tags too. So
//! the fork does not self-update: `update` changes nothing and says so, and
//! `doctor` does not look for or suggest newer releases. herdr-dash will take
//! over updates.

use std::path::PathBuf;

use anyhow::{Result, bail};

use crate::paths::{self, Ctx};

/// What `update` (and `update --check`) answers, word for word.
pub const NO_SELF_UPDATE: &str = "this fork does not self-update; herdr-dash will take over updates. Nothing was changed.";

/// The plugin root this binary was built in (`<root>/target/release/herdr-projects`).
pub fn own_root() -> Option<PathBuf> {
    let binary = paths::binary().ok()?;
    Some(binary.parent()?.parent()?.parent()?.to_path_buf())
}

/// Refuses, before asking Herdr, git or the network anything.
pub fn run(_ctx: &Ctx, _check_only: bool) -> Result<()> {
    bail!(NO_SELF_UPDATE)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::fake::FakeRunner;

    #[test]
    fn update_refuses_and_runs_nothing() {
        let home = tempfile::tempdir().unwrap();
        let env = crate::paths::Env::for_test(home.path(), &[]);
        let runner = FakeRunner::new();
        let ctx = Ctx { env: &env, root: home.path().join("root"), config_dir: home.path().join("cfg"), runner: &runner, detached_ticker: false };
        for check in [false, true] {
            let error = run(&ctx, check).unwrap_err().to_string();
            assert!(error.contains("this fork does not self-update; herdr-dash will take over updates"), "{error}");
        }
        assert!(runner.calls.borrow().is_empty(), "update must not call herdr, git or the network");
    }
}
