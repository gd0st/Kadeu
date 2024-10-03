use clap::Parser;
use kadeu::app::{self, CardBack, Deck};
use kadeu::cli::{self, Subcommand};
use kadeu::io::{convert_to_path, list_directory, FileType, ImportEntry};
use kadeu::tui::{App, Card};
use std::{default, env, io};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    deck: String,
    #[arg(long)]
    debug: bool,
}

fn fetch_root() -> String {
    if let Ok(var) = env::var("KADEU_HOME") {
        return var;
    }

    "".to_string()
}

fn main() -> io::Result<()> {
    // [X] TODO load the config here.
    let args = cli::Args::parse();
    let config = args.read_config()?;
    match args.subcommand {
        Subcommand::Run { name } => {
            let mut filepath = config.import_directory();

            name.split(".").for_each(|path| {
                filepath.push(path);
            });
            filepath.set_extension("json");

            let mut app = App::new().load(&filepath)?;
            if args.debug { app.with_debugger() } else { app }.run()?;
        }
        Subcommand::Source { path } => {
            if let Ok(deck) = FileType::json(&path).load::<Deck<Card>>() {
                let mut app = App::new().load(&path)?;
                if args.debug { app.with_debugger() } else { app }.run()?;
            } else {
                eprintln!("cannot parse");
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
            let deck: Deck<Card> = FileType::json(&path).load()?;
            let mut import_path = config.import_directory();
            import_path.push(filename);
            convert_to_path::<Deck<Card>>(FileType::json(&path), FileType::json(&import_path))?;
            // TODO some sort of file detection here.

            // TODO Card here is from tui. Need to rework the card system a bit.
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

    Ok(())

    // if let Ok(app) = App::new().load(&args.deck) {
    //     if args.debug { app.with_debugger() } else { app }.run()
    // } else {
    //     eprintln!("Could not find a deck at {}", args.deck);
    //     return Ok(());
    // }
}
