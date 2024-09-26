use clap::Parser;
use kadeu::app::{self, CardBack};
use kadeu::tui::App;
use std::{env, io};

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
    let args = Args::parse();

    if let Ok(app) = App::new().load(&args.deck) {
        if args.debug { app.with_debugger() } else { app }.run()
    } else {
        eprintln!("Could not find a deck at {}", args.deck);
        return Ok(());
    }
}
