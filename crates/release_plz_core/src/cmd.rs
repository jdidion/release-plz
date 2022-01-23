use std::process::Output;

use anyhow::Context;
use tracing::instrument;

#[instrument(skip_all)]
pub fn stdout(output: Output) -> anyhow::Result<String> {
    let stdout = String::from_utf8(output.stdout).context("cannot extract stdout")?;
    let stdout = stdout.trim();
    Ok(stdout.to_string())
}