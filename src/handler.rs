use crate::app::{App, AppResult, Penalty};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tui::style::Color;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        KeyCode::Char('h') | KeyCode::Char('H') => {
            if key_event.kind == crossterm::event::KeyEventKind::Press {
                app.toggle_help();
            }
        }

        KeyCode::Char(' ') => {
            if key_event.kind == crossterm::event::KeyEventKind::Release {
                app.handle_space();
                app.change_color(Color::White);
            } else if key_event.kind == crossterm::event::KeyEventKind::Press {
                if app.state != crate::app::State::Timing {
                    app.change_color(Color::Green);
                    app.show_help = false;
                }
            }
        }

        KeyCode::Char('s') | KeyCode::Char('S') => {
            if key_event.kind == crossterm::event::KeyEventKind::Press {
                app.new_scramble();
            }
        }

        KeyCode::Char('d') | KeyCode::Char('D') => {
            if key_event.kind == crossterm::event::KeyEventKind::Press {
                app.times.del_last();
            }
        }

        KeyCode::Char('r') | KeyCode::Char('R') => {
            if app.state == crate::app::State::Idle
                && key_event.kind == crossterm::event::KeyEventKind::Press
            {
                app.reset();
            }
        }

        KeyCode::Char('l') | KeyCode::Char('L') => {
            if key_event.kind == crossterm::event::KeyEventKind::Press {
                app.show_last_scramble = !app.show_last_scramble;
            }
        }

        KeyCode::Char('f') | KeyCode::Char('F') => {
            if key_event.kind == crossterm::event::KeyEventKind::Press {
                app.times.toggle_penalty(Penalty::DNF);
            }
        }

        KeyCode::Char('2') => {
            if key_event.kind == crossterm::event::KeyEventKind::Press {
                app.times.toggle_penalty(Penalty::PlusTwo);
            }
        }

        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
