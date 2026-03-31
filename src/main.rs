mod cli;
mod commands;
mod core;
mod menu;
mod output;

use cli::Cli;
use clap::Parser;

fn main() {
    let cli = Cli::parse();
    commands::execute(cli.command);
}
