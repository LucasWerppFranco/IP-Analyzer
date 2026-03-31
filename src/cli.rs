use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Calc { ip: String },
    Validate { ip: String },
    Info { ip: String },
    Subnet { ip: String },
}
