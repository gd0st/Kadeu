use clap::Parser;
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use kadeu::app::{self, CardBack};
use kadeu::tui::App;
use kadeu::ui::deck_browser::KadeuApp;
use kadeu::ui::inputs::{Events, Input, KeyMap};
use kadeu::ui::Action;
use kadeu::ui::{deck_browser::DeckBrowser, inputs::EventListener};
use ratatui::prelude::CrosstermBackend;
use ratatui::style::Stylize;
use ratatui::text::{Text, ToText};
use ratatui::widgets::{Block, List, ListState, Paragraph};
use ratatui::Terminal;
use std::io::stdout;
use std::path::PathBuf;
use std::{env, io};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    deck: Option<PathBuf>,
    #[arg(long)]
    debug: bool,
}

fn fetch_root() -> String {
    if let Ok(var) = env::var("KADEU_HOME") {
        return var;
    } else {
        ".".to_string()
    }
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let args = Args::parse();
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    let home_dir = PathBuf::from(fetch_root());

    let mut keymap = KeyMap::new();
    keymap.insert(KeyCode::Char('q'), Input::Quit);
    keymap.insert(KeyCode::Char('j'), Input::Down);
    keymap.insert(KeyCode::Char('k'), Input::Up);
    keymap.insert(KeyCode::Backspace, Input::Escape);
    keymap.insert(KeyCode::Enter, Input::Select);
    let mut browser = DeckBrowser::from(home_dir.clone());
    let events = Events::from(keymap);

    // event_bus.register(&mut browser);
    loop {
        let action = match browser.render(&mut terminal, &events) {
            Ok(action) => action,
            Err(e) => {
                eprintln!("{}", e.to_string());
                break;
            }
        };

        match action {
            Action::Quit => {
                break;
            }
            Action::Load(path) => loop {
                if let Ok(app) = App::new().load(&path) {
                    if args.debug { app.with_debugger() } else { app }.run()?;
                } else {
                    eprintln!("Could not find a deck at {}", path.to_string_lossy());
                    return Ok(());
                }
                break;
            },
            _ => {}
        }
    }

    terminal.clear()?;
    return disable_raw_mode();
}
