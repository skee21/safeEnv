mod cli;
mod config;
mod crypto;
mod git;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::GenerateKey => crypto::generate_key(),
        Commands::Init => git::init_vault(),
        Commands::AddUser { public_key } => (|| {
            config::add_public_key(&public_key)?;
            git::renormalize_tracked_files()
        })(),
        Commands::Track { filename } => git::track_file(&filename),
        Commands::Encrypt { stream: _ } => crypto::encrypt_stream(),
        Commands::Decrypt { stream: _ } => crypto::decrypt_stream(),
    };

    if let Err(e) = result {
        eprintln!("Error: {:#}", e);
        std::process::exit(1);
    }
}
