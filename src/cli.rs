use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "safeEnv", version, about = "Git-native decentralized secret vault")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    GenerateKey,
    Init,
    AddUser {
        public_key: String,
    },
    Encrypt {
        #[arg(long)]
        stream: bool,
    },
    Decrypt {
        #[arg(long)]
        stream: bool,
    },
}
