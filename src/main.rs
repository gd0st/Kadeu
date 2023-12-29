use clap::Parser;
use kadeu;
use kadeu::game::{Kadeu, Score, Sequence};
use kadeu::model::{Card, CardBack, CardSet};
use kadeu::sequences::{self, Linear};
use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead, Write};

#[derive(Parser, Debug)]
struct Config {
    #[clap(value_parser, default_value = "-")]
    filepath: String,
    #[clap(default_value = "shuffle")]
    sequence: String,
}

enum SequenceSelector {
    Linear,
    Random,
}

impl SequenceSelector {
    fn get(selector: String) -> Self {
        match selector.as_str() {
            "shuffle" => Self::Random,
            _ => Self::Linear,
        }
    }
}

fn main() {
    let args = Config::parse();
    let text = fs::read_to_string(args.filepath).unwrap();
    let set: CardSet<String, CardBack> = kadeu::from_str(text.as_str()).unwrap();
    let sequence = Linear::new(set.into_cards());

    let mut input = String::new();
    for card in sequence {
        print!(">{}", card.front());
        let mut stdin = io::stdin().lock();
        io::stdout().lock().flush();
        stdin.read_line(&mut input);
        print!(">>{}\n----------\n", card.back());
    }
}
