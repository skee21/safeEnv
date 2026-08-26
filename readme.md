# safeEnv

A decentralized, Git native secret management CLI. safeEnv leverages X25519 envelope encryption and Git's clean/smudge filters to transparently encrypt environment files during staging and decrypt them during checkout. 

## Installation

**Automatic Installation**
**Windows:** `irm https://raw.githubusercontent.com/skee21/safeEnv/main/install.ps1 | iex`
**Linux/Mac:** `curl -sSL https://raw.githubusercontent.com/skee21/safeEnv/main/install.sh | bash`

**Windows (Manual)**
1. Download `safeEnv.exe` from the latest GitHub Release.
2. Place the executable in a dedicated directory on your system.
3. Open your Windows Environment Variables and append that directory's path to your system `PATH` variable.
4. Restart your terminal.

**Linux & macOS (Source Build)**
You can build and install the tool directly from source using Cargo.
```bash
    git clone https://github.com/skee21/safeEnv.git
    cd safeEnv
    cargo install --path .
```
## Usage

Initialize the Git filters in your current repository.

    `safeEnv init`

Generate your local X25519 identity. The tool will output your public key and save your private key to your system configuration directory.

    `safeEnv generate-key`

Track a secret file. This configures Git to automatically intercept and encrypt this file upon staging.

   `safeEnv track .env`

Grant access to a collaborator. This adds their public key to the configuration and automatically re-encrypts all tracked files.

    `safeEnv add-user <PUBLIC_KEY>`