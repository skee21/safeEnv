use anyhow::Context;
use std::fs;
use std::io::Write;
use std::process::Command;

pub fn init_vault() -> anyhow::Result<()> {
    let check = Command::new("git")
        .args(["rev-parse", "--git-dir"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .context("Failed to run git")?;
    if !check.success() {
        anyhow::bail!("Not a git repository. Run 'git init' first.");
    }

    crate::config::save_vault_config(&crate::config::VaultConfig { keys: vec![] })?;
    eprintln!("Initialized empty vault configuration in .vault-config.json");

    let status = Command::new("git")
        .args(["config", "filter.vault.clean", "safeEnv encrypt --stream"])
        .status()
        .context("Failed to run git config for filter.vault.clean")?;
    if !status.success() {
        anyhow::bail!("Failed to set git filter.vault.clean");
    }
    eprintln!("Configured git filter.vault.clean");

    let status = Command::new("git")
        .args(["config", "filter.vault.smudge", "safeEnv decrypt --stream"])
        .status()
        .context("Failed to run git config for filter.vault.smudge")?;
    if !status.success() {
        anyhow::bail!("Failed to set git filter.vault.smudge");
    }
    eprintln!("Configured git filter.vault.smudge");

    let status = Command::new("git")
        .args(["config", "filter.vault.required", "true"])
        .status()
        .context("Failed to run git config for filter.vault.required")?;
    if !status.success() {
        anyhow::bail!("Failed to set git filter.vault.required");
    }
    eprintln!("Configured git filter.vault.required");

    eprintln!("safeEnv vault initialization complete.");
    Ok(())
}

pub fn track_file(filename: &str) -> anyhow::Result<()> {
    let path = std::path::Path::new(".gitattributes");
    let existing = if path.exists() {
        fs::read_to_string(path).context("Failed to read .gitattributes")?
    } else {
        String::new()
    };

    for line in existing.lines() {
        if line.split_whitespace().next() == Some(filename) {
            eprintln!("{} is already tracked by safeEnv", filename);
            return Ok(());
        }
    }

    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .context("Failed to open .gitattributes for writing")?;

    if !existing.is_empty() && !existing.ends_with('\n') {
        writeln!(file).context("Failed to write newline to .gitattributes")?;
    }

    writeln!(file, "{} filter=vault diff=vault -text", filename)
        .context("Failed to write to .gitattributes")?;

    eprintln!("Tracking {} with safeEnv vault filter", filename);
    Ok(())
}

pub fn renormalize_tracked_files() -> anyhow::Result<()> {
    let path = std::path::Path::new(".gitattributes");
    if !path.exists() {
        eprintln!("No .gitattributes found, skipping re-encryption");
        return Ok(());
    }

    let content = fs::read_to_string(path).context("Failed to read .gitattributes")?;

    let tracked: Vec<&str> = content
        .lines()
        .filter(|line| line.contains("filter=vault"))
        .filter_map(|line| line.split_whitespace().next())
        .collect();

    if tracked.is_empty() {
        eprintln!("No files tracked with vault filter, skipping re-encryption");
        return Ok(());
    }

    for filename in &tracked {
        let status = Command::new("git")
            .args(["add", "--renormalize", filename])
            .status()
            .with_context(|| format!("Failed to run git add --renormalize {}", filename))?;
        if !status.success() {
            eprintln!("Warning: git add --renormalize {} exited with non-zero status", filename);
        } else {
            eprintln!("Re-encrypted {}", filename);
        }
    }

    eprintln!("Re-encryption complete for {} file(s)", tracked.len());
    Ok(())
}
