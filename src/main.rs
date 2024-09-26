use clap::Parser;
use kadeu::app::{self, CardBack};
use kadeu::tui::App;
use std::io;
type Card = app::Card<String, CardBack>;
type Deck = app::Deck<Card>;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    deck: String,
    #[arg(long)]
    debug: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let mut app = App::new();
    if let Ok(()) = app.load(&args.deck) {
    } else {
        eprintln!("Could not find a deck at {}", args.deck);
        return Ok(());
    };
    if args.debug { app.with_debugger() } else { app }.run()
}
