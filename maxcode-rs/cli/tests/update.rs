use anyhow::Result;
use predicates::str::contains;
use std::path::Path;
use tempfile::TempDir;

fn maxcode_command(maxcode_home: &Path) -> Result<assert_cmd::Command> {
    let mut cmd = assert_cmd::Command::new(maxcode_utils_cargo_bin::cargo_bin("maxcode")?);
    cmd.env("MAXCODE_HOME", maxcode_home);
    Ok(cmd)
}

#[cfg(debug_assertions)]
#[tokio::test]
async fn update_does_not_start_interactive_prompt() -> Result<()> {
    let maxcode_home = TempDir::new()?;

    maxcode_command(maxcode_home.path())?
        .arg("update")
        .assert()
        .failure()
        .stderr(contains("`maxcode update` is not available in debug builds"));

    Ok(())
}
