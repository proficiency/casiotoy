use anyhow::Result;
use crossterm::{
    ExecutableCommand,
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::prelude::*;
use std::env;
use std::io::stdout;

mod display;
mod settings;
mod time;
mod watch;

use watch::{Watch, WatchModel};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let model = if args.len() > 1 && args[1] == "f91w" {
        WatchModel::AE1200
    } else {
        WatchModel::F91W
    };

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut watch = Watch::new(model)?;
    run_app(&mut terminal, &mut watch)?;

    // clean up
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    watch: &mut Watch,
) -> Result<()> {
    loop {
        terminal.draw(|f| display::ui(f, watch))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if handle_key_event(key, watch)? {
                    return Ok(());
                }
            }
        }

        // tick
        watch.update()?;
    }
}

fn handle_key_event(key: KeyEvent, watch: &mut Watch) -> Result<bool> {
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => return Ok(true),
        KeyCode::Char('m') => watch.toggle_mode()?,
        KeyCode::Char('s') => watch.toggle_start_stop()?,
        KeyCode::Char('r') => watch.reset()?,
        KeyCode::Char('l') => watch.toggle_light()?,
        KeyCode::Char('a') => watch.set_alarm()?,
        _ => {}
    }
    Ok(false)
}
