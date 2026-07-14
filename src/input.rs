use crossterm::event::KeyEvent;
use crate::app::{App, Screen};

pub fn handle_event(key: KeyEvent, app: &mut App) -> bool {
    use crossterm::event::{KeyCode, KeyModifiers};

    if key.modifiers.contains(KeyModifiers::CONTROL) {
        match key.code {
            KeyCode::Char('c') => return false,
            KeyCode::Char('r') => {
                app.restart();
                return true;
            }
            _ => {}
        }
    }

    match app.screen {
        Screen::Menu => handle_menu_event(key, app),
        Screen::Typing => handle_typing_event(key, app),
        Screen::Results => handle_results_event(key, app),
    }

    true
}

fn handle_menu_event(key: KeyEvent, app: &mut App) {
    use crossterm::event::KeyCode;
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => app.menu_up(),
        KeyCode::Down | KeyCode::Char('j') => app.menu_down(),
        KeyCode::Tab => app.next_theme(),
        KeyCode::Enter => {
            let mode = app.selected_mode();
            app.start_test(mode);
        }
        KeyCode::Char('q') | KeyCode::Esc => {
            app.quit = true;
        }
        _ => {}
    }
}

fn handle_typing_event(key: KeyEvent, app: &mut App) {
    use crossterm::event::KeyCode;
    if let Some(ref mut test) = app.typing_test {
        match key.code {
            KeyCode::Char(c) => test.type_char(c),
            KeyCode::Backspace => test.backspace(),
            KeyCode::Esc => app.back_to_menu(),
            _ => {}
        }
    }
}

fn handle_results_event(key: KeyEvent, app: &mut App) {
    use crossterm::event::KeyCode;
    match key.code {
        KeyCode::Enter => app.restart(),
        KeyCode::Esc => app.back_to_menu(),
        KeyCode::Char('q') => app.back_to_menu(),
        _ => {}
    }
}
