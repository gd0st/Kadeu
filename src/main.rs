use clap::Parser;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use kadeu::app::Deck;
use kadeu::cli::{self, Subcommand};
use kadeu::game::engine::ui::FlashcardApp;
use kadeu::io::{convert_to_path, list_directory, FileType, ImportEntry};
// use kadeu::tui::{App, Card};
use kadeu::ui::deck_browser::DeckBrowser;
use kadeu::ui::{AppHandler, Exit};
use kadeu::Flashcard;
use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use std::io::{stdout, Stdout};

// Governs how subcollection flashcard should be accessed.
const IFS: &str = "/";

enum Apps {
    DeckBrowser,
    FlashcardApp,
}

fn crossterm_terminal() -> std::io::Result<Terminal<CrosstermBackend<Stdout>>> {
    Terminal::new(CrosstermBackend::new(stdout()))
}

fn main() -> io::Result<()> {
    let args = cli::Args::parse();
    let mut subcommand = args.subcommand.clone().unwrap_or_default();

    //let mut browser = None;
    let config = args.read_config()?;
    let mut browser = DeckBrowser::try_from(config.import_directory())?;
    enable_raw_mode()?;
    let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut app = AppHandler::from(terminal);

    loop {
        match &subcommand {
            Subcommand::Browse => {
                // makes a new browser if none exists
                // otherwise uses whatever browser has already been produced
                // prevents the UI from resetting to root after the user finishes browsing.

                let _action = app.run(&mut browser)?;
                if let Exit::Quit = _action {
                    break;
                }

                if browser.is_deck::<Flashcard>() {
                    let path = browser.current_path();
                    subcommand = Subcommand::Source { path }
                }
            }
            Subcommand::Run { name } => {
                let mut filepath = config.import_directory();
                name.split(IFS).for_each(|path| {
                    filepath.push(path);
                });
                filepath.set_extension("json");
                subcommand = Subcommand::Source { path: filepath };
            }
            Subcommand::Source { path } => {
                let deck: Deck<Flashcard> = FileType::json(&path).load()?;
                let mut flashcard_app = FlashcardApp::from(deck);
                let action = app.run(&mut flashcard_app)?;

                if let Exit::Quit = action {
                    break;
                }

                if let Exit::Drop = action {
                    subcommand = Subcommand::Browse
                }
            }
            Subcommand::Import { path } => {
                if !path.is_file() {
                    eprintln!("path is not a file");
                    return Ok(());
                }

                let Some(filename) = path.file_name() else {
                    eprintln!("is a file but has no name?");
                    return Ok(());
                };
                let mut import_path = config.import_directory();
                import_path.push(filename);
                convert_to_path::<Deck<Flashcard>>(
                    FileType::json(&path),
                    FileType::json(&import_path),
                )?;
                // TODO some sort of file detection here.
            }

            Subcommand::Show => {
                let imports_directory = config.import_directory();
                let entries = list_directory(&imports_directory)?;
                for entry in entries {
                    match entry {
                        ImportEntry::Collection(path) => {
                            println!("{}/", path.file_stem().unwrap().to_string_lossy());
                        }
                        ImportEntry::File(path) => {
                            println!("{}", path.file_stem().unwrap().to_string_lossy());
                        }
                    }
                }
            }
            _ => {}
        }
    }
    disable_raw_mode()?;

    Ok(())
}
