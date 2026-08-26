use age::secrecy::ExposeSecret;
use std::fs;
use std::io;

pub fn generate_key() -> anyhow::Result<()> {
    let identity = age::x25519::Identity::generate();
    let recipient = identity.to_public();

    let path = crate::config::identity_file_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&path, identity.to_string().expose_secret())?;

    eprintln!("Public key: {}", recipient);
    eprintln!("Identity saved to {}", path.display());
    Ok(())
}

pub fn encrypt_stream() -> anyhow::Result<()> {
    let config = crate::config::load_vault_config()?;
    if config.keys.is_empty() {
        anyhow::bail!("No public keys found in .vault-config.json. Add at least one recipient with 'safeEnv add-user <public_key>'");
    }

    let recipients: Vec<age::x25519::Recipient> = config
        .keys
        .iter()
        .map(|k| k.parse::<age::x25519::Recipient>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| anyhow::anyhow!("Failed to parse recipient key: {}", e))?;

    let encryptor = age::Encryptor::with_recipients(
        recipients.iter().map(|r| r as &dyn age::Recipient),
    )
    .map_err(|e| anyhow::anyhow!("Failed to create encryptor: {}", e))?;

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin_lock = stdin.lock();
    let mut stdout_lock = stdout.lock();

    let mut writer = encryptor
        .wrap_output(&mut stdout_lock)
        .map_err(|e| anyhow::anyhow!("Failed to wrap output: {}", e))?;

    io::copy(&mut stdin_lock, &mut writer)?;
    writer
        .finish()
        .map_err(|e| anyhow::anyhow!("Failed to finalize encryption: {}", e))?;

    Ok(())
}

pub fn decrypt_stream() -> anyhow::Result<()> {
    let path = crate::config::identity_file_path()?;
    if !path.exists() {
        anyhow::bail!(
            "Identity file not found at {}. Run 'safeEnv generate-key' first.",
            path.display()
        );
    }

    let key_str = fs::read_to_string(&path)?;
    let identity = key_str
        .trim()
        .parse::<age::x25519::Identity>()
        .map_err(|e| anyhow::anyhow!("Failed to parse identity: {}", e))?;

    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_lock = stdin.lock();
    let mut stdout_lock = stdout.lock();

    let decryptor = age::Decryptor::new(stdin_lock)
        .map_err(|e| anyhow::anyhow!("Failed to initialize decryptor: {}", e))?;

    let mut reader = decryptor
        .decrypt(std::iter::once(&identity as &dyn age::Identity))
        .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))?;

    io::copy(&mut reader, &mut stdout_lock)?;
    Ok(())
}
