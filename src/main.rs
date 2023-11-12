use clap::{Parser, Subcommand};
use kadeu;
use kadeu::game::{Kadeu, Score, Sequence};
use kadeu::model::{Card, CardBack, CardSet};
use kadeu::sequences::{Linear, Random};
use std::fs;
use std::io::{self, BufRead, Write};
use std::iter::Peekable;

#[derive(Parser, Debug)]
struct Config {
    #[clap(value_parser, default_value = "-")]
    filepath: String,
    #[clap(default_value = "shuffle")]
    sequence: String,
    take: Option<usize>,
}

#[derive(Subcommand, Debug, Clone)]
enum SequenceSelector {
    Linear,
    Random,
}

enum Mode {
    Practice,
    Test,
}

struct App<T> {
    items: Vec<T>,
    mode: Mode,
}

impl<T> App<T> {
    pub fn new(items: Vec<T>, mode: Mode) -> Self {
        Self { items, mode }
    }
}

impl<T> App<T>
where
    T: Kadeu,
{
    fn next_card(&mut self) -> Option<T> {
        self.items.pop()
    }

    fn eval(&self, card: &T, answer: &String) -> Option<Score> {
        match (&self.mode, card.eval(answer)) {
            (Mode::Practice, _) => None,
            (_, Some(score)) => Some(score),
            (_, None) => None,
        }
    }
}

impl SequenceSelector {
    fn get(key: &str) -> Self {
        match key {
            "linear" => Self::Linear,
            "shuffle" => Self::Random,
            _ => Self::Random,
        }
    }

    fn sequence<T>(&self, items: Vec<T>) -> Vec<T> {
        match &self {
            SequenceSelector::Random => Random::new(items).into_iter(),
            SequenceSelector::Linear => Linear::new(items).into_iter(),
        }
        .collect()
    }
}

fn main() {
    let args = Config::parse();
    let text = fs::read_to_string(args.filepath).unwrap();
    // TODO error handling for this.
    let set: CardSet<String, CardBack> =
        kadeu::from_str(text.as_str()).expect("Kadeu failed to parse the specified filepath.");
    let sequence = SequenceSelector::get(&args.sequence);
    let cards = sequence.sequence(set.into_cards());

    let mut app = App::new(cards, Mode::Practice);
    let mut input = String::new();
    while let Some(card) = app.next_card() {
        print!(">{}", card.prompt());
        let mut stdin = io::stdin().lock();
        io::stdout().lock().flush();
        stdin.read_line(&mut input);
        if let Some(score) = app.eval(&card, &input) {
            println! {"{}!", score.to_string()}
        }
        print!(">>{}\n----------\n", card.back());
    }
}
