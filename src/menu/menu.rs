use std::io::{stdout, Write};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{self},
};
use crate::menu::item::{MenuItem, MenuAction};

pub fn run_menu(items: Vec<MenuItem>) -> std::io::Result<()> {
    let mut selected = 0;

    terminal::enable_raw_mode()?;

    // Store cursor position before menu
    let (start_col, start_row) = cursor::position()?;

    // Calculate menu dimensions
    let menu_width: u16 = 40;
    let menu_height: u16 = items.len() as u16 + 5; // header + items + padding + instructions

    // Draw menu frame initially
    draw_menu(&items, selected, start_col, start_row, menu_width)?;

    loop {
        // Redraw menu with updated selection
        draw_menu(&items, selected, start_col, start_row, menu_width)?;

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
                    // Move cursor below the menu before executing action
                    execute!(stdout(), cursor::MoveTo(0, start_row + menu_height + 1))?;

                    // Disable raw mode before executing action to allow normal input
                    terminal::disable_raw_mode()?;
                    let action_result = (items[selected].action)();
                    // Re-enable raw mode to continue menu
                    terminal::enable_raw_mode()?;

                    if matches!(action_result, MenuAction::Exit) {
                        // Move cursor below the menu output without clearing
                        let (_, end_row) = cursor::position()?;
                        execute!(stdout(), cursor::MoveTo(0, end_row + 1))?;
                        break;
                    }

                    // Get new cursor position after action
                    let (_, new_row) = cursor::position()?;

                    // Redraw menu below the output
                    draw_menu(&items, selected, start_col, new_row + 1, menu_width)?;
                }
                KeyCode::Esc => {
                    // Move cursor below the menu without clearing
                    execute!(stdout(), cursor::MoveTo(0, start_row + menu_height + 1))?;
                    break;
                }
                _ => {}
            }
        }
    }

    terminal::disable_raw_mode()?;
    Ok(())
}

fn draw_menu(items: &[MenuItem], selected: usize, col: u16, row: u16, width: u16) -> std::io::Result<()> {
    let mut stdout = stdout();

    // Draw top border
    execute!(stdout, cursor::MoveTo(col, row))?;
    print!("┌{:─^width$}┐", "", width = width as usize - 2);

    // Draw title
    execute!(stdout, cursor::MoveTo(col, row + 1))?;
    let title = " MENU ";
    let padding = (width as usize - 2 - title.len()) / 2;
    print!("│{: >padding$}{title}{: <padding$}│", "", "", padding = padding);

    // Draw separator
    execute!(stdout, cursor::MoveTo(col, row + 2))?;
    print!("├{:─^width$}┤", "", width = width as usize - 2);

    // Draw items
    for (i, item) in items.iter().enumerate() {
        execute!(stdout, cursor::MoveTo(col, row + 3 + i as u16))?;
        let label = &item.label;
        if i == selected {
            print!("│  ▶ {: <width$}│", label, width = width as usize - 6);
        } else {
            print!("│    {: <width$}│", label, width = width as usize - 6);
        }
    }

    // Draw bottom border
    execute!(stdout, cursor::MoveTo(col, row + 3 + items.len() as u16))?;
    print!("└{:─^width$}┘", "", width = width as usize - 2);

    // Draw instructions
    execute!(stdout, cursor::MoveTo(col, row + 4 + items.len() as u16))?;
    print!("  [↑/↓] Navigate  [Enter] Select  [Esc] Exit");

    stdout.flush()
}
