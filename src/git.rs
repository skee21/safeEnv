use std::process::Command;

pub fn init_vault() -> anyhow::Result<()> {
    crate::config::save_vault_config(&crate::config::VaultConfig { keys: vec![] })?;
    eprintln!("Initialized empty vault configuration in .vault-config.json");

    let status = Command::new("git")
        .args(["config", "filter.vault.clean", "safeEnv encrypt --stream"])
        .status()?;
    if !status.success() {
        anyhow::bail!("Failed to set git filter.vault.clean");
    }
    eprintln!("Configured git filter.vault.clean");

    let status = Command::new("git")
        .args(["config", "filter.vault.smudge", "safeEnv decrypt --stream"])
        .status()?;
    if !status.success() {
        anyhow::bail!("Failed to set git filter.vault.smudge");
    }
    eprintln!("Configured git filter.vault.smudge");

    let status = Command::new("git")
        .args(["config", "filter.vault.required", "true"])
        .status()?;
    if !status.success() {
        anyhow::bail!("Failed to set git filter.vault.required");
    }
    eprintln!("Configured git filter.vault.required");

    eprintln!("safeEnv vault initialization complete.");
    Ok(())
}
