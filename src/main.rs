use clap::Parser;
use kadeu::app::{self, CardBack};
use kadeu::tui::App;
use std::io;
type Card = app::Card<String, CardBack>;
type Deck = app::Deck<Card>;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    from: Option<String>,
    #[arg(long)]
    debug: bool,
}
#[derive(Debug, Clone)]
enum Action {
    Quit,
    Next,
    Restart,
    Continue,
}

fn main() -> io::Result<()> {
    let deck_str = r#"
{
	"title": "Foobar Deck",
	"cards": [
		{ "front": "Foo", "back": "Bar" },
		{ "front": "Bizz", "back": "bazz" }
	]
}
	"#;

    let args = Args::parse();
    let deck = Deck::try_from(deck_str)?;
    let mut app = App::new();
    if let Some(filename) = args.from {
        app.load(filename)?;
    } else {
        app.set_deck(deck);
    }

    if args.debug { app.with_debugger() } else { app }.run()
}
