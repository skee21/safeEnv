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
## Usage Workflow

safeEnv is designed to be completely invisible once set up. Here is a typical team workflow.

### 1. Initial Setup (Project Lead)
When starting a new project or migrating an existing one, the project lead needs to initialize the vault and generate their key:
1. Run `safeEnv init` inside the Git repository. This configures the local repository to use safeEnv's encryption filters.
2. Run `safeEnv generate-key`. Your private key is saved securely to your system, and your public key is printed to the terminal.
3. Add your public key to the project by running `safeEnv add-user <YOUR_PUBLIC_KEY>`.
4. Tell Git which files contain secrets by running `safeEnv track <filename>` (for example, `safeEnv track .env`). 

After this, you can just `git add .env` and `git commit` normally. Git will seamlessly encrypt the file in the background before saving it to history.

### 2. Onboarding Teammates
When a new developer joins the project, they need to generate their own identity and request access:
1. The new developer clones the repository.
2. They run `safeEnv generate-key` and send the resulting public key to the project lead (via Slack, email, etc.).
3. The project lead runs `safeEnv add-user <NEW_PUBLIC_KEY>`. This adds the new user to `.vault-config.json` and automatically triggers a Git re-normalization, which re-encrypts all tracked secrets to include the new teammate.
4. The project lead commits and pushes the changes.
5. The new developer pulls the latest `main` branch, and their local Git will seamlessly decrypt the secret files using their new identity.

### Things to Keep in Mind
* **Never commit your private key:** Your identity file lives in your system's global AppData or configuration directory, far away from your Git repositories. 
* **Filters must be initialized:** Anyone interacting with the repository must run `safeEnv init` locally. If they do not, Git will not know how to decrypt the files.

## How It Works

safeEnv is designed to make managing secrets in Git repositories completely transparent and decentralized. Instead of relying on a central server to store your secrets, safeEnv encrypts the files directly on your machine before they ever enter your Git history.

### Git Filters
The core of safeEnv integrates with Git's native clean and smudge filters. When you initialize safeEnv and track a file, Git is instructed to run the file through safeEnv whenever you stage changes or check out a branch. 
* **Clean Filter (Encryption):** When you run `git add`, Git pipes the plain text file through safeEnv. safeEnv encrypts it, and Git only stores the encrypted bytes.
* **Smudge Filter (Decryption):** When you run `git checkout`, Git pipes the encrypted bytes through safeEnv. safeEnv decrypts it using your local identity and places the plain text file in your working directory.

### Envelope Encryption
The encryption is handled using X25519 public key cryptography. When you generate a key, your private key stays securely on your local machine. You share your public key with your team. 

The public keys of everyone who needs access are stored in a `.vault-config.json` file in your repository. When safeEnv encrypts a file, it encrypts it so that any of the public keys in that configuration file can decrypt it. If you need to add a new team member, you simply add their public key and the files are automatically re-encrypted to include them.
 
## Security Guarantees

safeEnv is built to ensure your secrets are never accidentally exposed, even if someone makes a mistake.

* **Zero Private Key Exposure:** Your private X25519 identity is generated directly into your operating system's native application data directory. It never exists in the project folder, so it cannot be accidentally committed or pushed. The repository only stores public keys inside `.vault-config.json`, which are completely safe to share.
* **Automatic Transparent Encryption:** Because the encryption is tied to Git's native "clean" filter, it happens automatically during `git add`. You do not have to remember to run an encryption command before committing. Git intercepts the file, passes it to safeEnv, and only writes the ciphertext to the staging area.
* **Automated Re-encryption:** When you grant a new user access using `safeEnv add-user`, the CLI automatically parses your `.gitattributes` file to find all tracked secrets and executes a `git add --renormalize` on them. This forces Git to re-process the files through the encryption engine immediately, ensuring the new user has access without requiring manual updates to the files.
* **Fallback Protection:** If you attempt to decrypt a file but lack the correct identity, the process fails safely and clearly. It will not panic or corrupt your working directory.