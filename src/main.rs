use anyhow::{Context, Result};
use mime_guess::MimeGuess;
use std::process::Command;

fn main() -> Result<()> {
    let url = std::env::args().nth(1).context("expected url")?;
    let ext = url.rsplit_once(".").unwrap().1;
    let mime = MimeGuess::from_ext(ext).first_or_octet_stream();
    let output = Command::new("xdg-mime")
        .arg("query")
        .arg("default")
        .arg(mime.essence_str())
        .output()?;
    if !output.status.success() {
        anyhow::bail!(
            "xdg-mime returned status {}: {}",
            output.status,
            std::str::from_utf8(&output.stderr)?
        );
    }
    let desktop = std::str::from_utf8(&output.stdout)?.trim();
    let output = Command::new("gtk-launch").arg(desktop).arg(url).output()?;
    if !output.status.success() {
        anyhow::bail!(
            "gtk-launch returned status {}: {}",
            output.status,
            std::str::from_utf8(&output.stderr)?
        );
    }
    Ok(())
}
