use std::io::{stdout, Write};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, ClearType},
};
use crate::menu::item::MenuItem;

pub fn run_menu(items: Vec<MenuItem>) -> std::io::Result<()> {
    let mut selected = 0;

    terminal::enable_raw_mode()?;

    loop {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))?;

        println!("=== MENU ===\n");

        for (i, item) in items.iter().enumerate() {
            if i == selected {
                println!("> {}", item.label);
            } else {
                println!("  {}", item.label);
            }
        }

        stdout().flush()?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('w') | KeyCode::Up => {
                    if selected == 0 {
                        selected = items.len() - 1;
                    } else {
                        selected -= 1;
                    }
                }
                KeyCode::Char('s') | KeyCode::Down => {
                    selected = (selected + 1) % items.len();
                }
                KeyCode::Enter => {
                    (items[selected].action)();
                }
                KeyCode::Esc => break,
                _ => {}
            }
        }
    }

    terminal::disable_raw_mode()?;
    Ok(())
}
