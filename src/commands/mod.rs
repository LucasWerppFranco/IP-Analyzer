pub mod subnet;

use crate::cli::Commands;

pub fn execute(cli: Commands) {
    match cli {
        Commands::Calc { ip } => {
            println!("Calc command with IP: {}", ip);
        }
        Commands::Validate { ip } => {
            println!("Validate command with IP: {}", ip);
        }
        Commands::Info { ip } => {
            println!("Info command with IP: {}", ip);
        }
        Commands::Subnet { ip } => {
            subnet::run(ip);
        }
    }
}
