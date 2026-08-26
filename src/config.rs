use std::fs;
use std::path::PathBuf;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct VaultConfig {
    pub keys: Vec<String>,
}

pub fn identity_file_path() -> anyhow::Result<PathBuf> {
    if let Ok(dir) = std::env::var("SAFEENV_CONFIG_DIR") {
        return Ok(PathBuf::from(dir).join("identity.txt"));
    }
    let proj_dirs = directories::ProjectDirs::from("", "", "safeEnv")
        .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;
    Ok(proj_dirs.config_dir().join("identity.txt"))
}

pub fn load_vault_config() -> anyhow::Result<VaultConfig> {
    let path = std::path::Path::new(".vault-config.json");
    if !path.exists() {
        anyhow::bail!("Vault configuration file (.vault-config.json) not found in current directory");
    }
    let content = fs::read_to_string(path)?;
    let config: VaultConfig = serde_json::from_str(&content)?;
    Ok(config)
}

pub fn save_vault_config(config: &VaultConfig) -> anyhow::Result<()> {
    let json = serde_json::to_string_pretty(config)?;
    fs::write(".vault-config.json", json)?;
    Ok(())
}

pub fn add_public_key(key: &str) -> anyhow::Result<()> {
    let mut config = load_vault_config()?;
    config.keys.push(key.to_string());
    save_vault_config(&config)?;
    Ok(())
}
