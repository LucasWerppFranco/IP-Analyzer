pub mod subnet;

use crate::cli::Commands;
use crate::menu::{run_menu, item::{MenuItem, MenuAction}};
use crate::output::print_subnet_result;
use crate::core::subnet as core_subnet;
use crate::core::ip;
use crossterm::style::{Color, Stylize};

fn menu_calc() -> MenuAction {
    use std::io::{self, Write};

    println!("Enter IP address to calculate:");
    println!("(This feature will be implemented soon)");
    println!();
    println!("Press Enter to continue...");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut String::new()).unwrap();
    MenuAction::Continue
}

fn menu_validate() -> MenuAction {
    use std::io::{self, Write};

    println!("Enter IP address to validate:");
    println!("(This feature will be implemented soon)");
    println!();
    println!("Press Enter to continue...");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut String::new()).unwrap();
    MenuAction::Continue
}

fn menu_info() -> MenuAction {
    use std::io::{self, Write};

    println!("Enter IP address for info:");
    println!("(This feature will be implemented soon)");
    println!();
    println!("Press Enter to continue...");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut String::new()).unwrap();
    MenuAction::Continue
}

fn menu_subnet() -> MenuAction {
    use std::io::{self, Write};

    print!("Enter IP with CIDR (e.g., 192.168.1.0/24): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    match ip::parse(input) {
        Ok(ip_with_cidr) => {
            let result = core_subnet::calculate(ip_with_cidr);
            println!();
            println!("{}", "=== Subnet Information ===".with(Color::Cyan));
            print_subnet_result(&result);
        }
        Err(e) => {
            println!("{}", format!("Error: {}", e).with(Color::Red));
        }
    }

    println!();
    println!("Press Enter to continue...");
    io::stdin().read_line(&mut String::new()).unwrap();
    MenuAction::Continue
}

fn menu_exit() -> MenuAction {
    println!("Exiting menu...");
    MenuAction::Exit
}

fn show_interactive_menu() {
    let items = vec![
        MenuItem {
            label: "SUBNET CALCULATION".to_string(),
            action: menu_subnet,
        },
        MenuItem {
            label: "CALCULATE IP".to_string(),
            action: menu_calc,
        },
        MenuItem {
            label: "VALIDATE IP".to_string(),
            action: menu_validate,
        },
        MenuItem {
            label: "IP INFO".to_string(),
            action: menu_info,
        },
        MenuItem {
            label: "EXIT".to_string(),
            action: menu_exit,
        },
    ];

    if let Err(e) = run_menu(items) {
        eprintln!("Menu error: {}", e);
    }
}

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
        Commands::Menu => {
            show_interactive_menu();
        }
    }
}
