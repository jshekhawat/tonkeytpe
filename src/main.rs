mod app;
mod config;
mod input;
mod stats;
mod theme;
mod tui;
mod typing;
mod words;

use std::{io, time::Duration};

use app::App;
use config::Config;
use crossterm::{
    event::{self, EnableMouseCapture, Event},
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
};
use input::handle_event;
use ratatui::backend::CrosstermBackend;
use theme::THEMES;
use tui::render;

fn main() -> io::Result<()> {
    let mut terminal = setup_terminal()?;
    let tick_rate = Duration::from_millis(20);

    let config = Config::load();
    let mut app = App::new();

    if let Some(theme) = THEMES.iter().find(|t| t.name == config.theme) {
        app.theme = *theme;
    }

    loop {
        terminal.draw(|f| render(f, &app))?;

        if event::poll(tick_rate)? {
            if let Event::Key(key) = event::read()? {
                if !handle_event(key, &mut app) {
                    break;
                }
            }
        }

        if app.quit {
            break;
        }

        app.tick();
    }

    let config = Config {
        theme: app.theme.name.to_string(),
        ..config
    };
    config.save();

    teardown_terminal(&mut terminal)
}

fn setup_terminal() -> io::Result<ratatui::Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    ratatui::Terminal::new(backend)
}

fn teardown_terminal(
    terminal: &mut ratatui::Terminal<CrosstermBackend<io::Stdout>>,
) -> io::Result<()> {
    use crossterm::{
        event::DisableMouseCapture,
        execute,
        terminal::{disable_raw_mode, LeaveAlternateScreen},
    };
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()
}
