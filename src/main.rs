mod cli;
mod commands;
mod core;
mod menu;
mod output;

use cli::Cli;
use clap::Parser;
use crossterm::style::{Color, Stylize};
use std::io::{self, Write};

const ASCII_ART: &str = include_str!("../assets/art.txt");

fn show_welcome_screen() {
    // Print ASCII art in blue
    println!("{}", ASCII_ART.with(Color::Blue));
    println!("-------------------------------------------------------------------------------------------------------");
    println!("|                                     Welcome to IP-Analyzer!!!                                       |");
    println!("|                     If you need help, type \"help\" or press Ctrl+C to exit.                          |");
    println!("-------------------------------------------------------------------------------------------------------");
    println!();
}

fn run_interactive_mode() {
    show_welcome_screen();

    loop {
        println!("───────────────────────────────────────────────────────────────────────────────────────────────────────");
        print!("❯ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        println!("───────────────────────────────────────────────────────────────────────────────────────────────────────");

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        let command = parts[0].to_lowercase();

        match command.as_str() {
            "help" | "h" => {
                println!("Available commands:");
                println!("  subnet <ip>/<prefix>  - Calculate subnet information");
                println!("  calc <ip>             - Perform calculations on an IP");
                println!("  validate <ip>         - Validate an IP address format");
                println!("  info <ip>             - Display information about an IP");
                println!("  help                  - Show this help message");
                println!("  exit, quit            - Exit the program");
                println!();
            }
            "exit" | "quit" | "q" => {
                println!("{}", "Goodbye!".with(Color::Green));
                break;
            }
            "subnet" => {
                if parts.len() < 2 {
                    println!("{}", "Usage: subnet <ip>/<prefix>".with(Color::Red));
                } else {
                    commands::subnet::run(parts[1].to_string());
                }
            }
            "calc" => {
                if parts.len() < 2 {
                    println!("{}", "Usage: calc <ip>".with(Color::Red));
                } else {
                    println!("{}", format!("Calc command with IP: {}", parts[1]).with(Color::Green));
                }
            }
            "validate" => {
                if parts.len() < 2 {
                    println!("{}", "Usage: validate <ip>".with(Color::Red));
                } else {
                    println!("{}", format!("Validate command with IP: {}", parts[1]).with(Color::Green));
                }
            }
            "info" => {
                if parts.len() < 2 {
                    println!("{}", "Usage: info <ip>".with(Color::Red));
                } else {
                    println!("{}", format!("Info command with IP: {}", parts[1]).with(Color::Green));
                }
            }
            _ => {
                println!("{}", format!("Unknown command: '{}'. Type 'help' for available commands.", command).with(Color::Red));
            }
        }

        println!();
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        // No arguments provided, run interactive mode
        run_interactive_mode();
    } else {
        // Arguments provided, use clap parsing
        let cli = Cli::parse();
        commands::execute(cli.command);
    }
}
