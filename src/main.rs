use clap::Parser;
use kadeu::game::{Kadeu, Score};
use kadeu::model::{Card, CardBack, CardSet};
use kadeu::strategy;
use kadeu::strategy::Strategy;
use std::collections::HashMap;
use std::fs;
use std::io::Result;
use std::io::{self, BufRead, Write};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Config {
    #[clap(value_parser, default_value = "-", required = true)]
    filepath: String,
    #[clap(default_value = "shuffle")]
    feeder: String,
}

enum FeederSelector {
    Linear,
    Random,
}

impl FeederSelector {
    fn get(selector: String) -> Self {
        match selector.as_str() {
            "shuffle" => Self::Random,
            _ => Self::Linear,
        }
    }
}

fn main() -> Result<()> {
    let args = Config::parse();
    let text = fs::read_to_string(args.filepath)?;
    let set: CardSet<Card<String, CardBack>> = CardSet::try_from(text.as_str())?;
    let mut cards = set.cards();
    let strategy = strategy::Random;
    let mut input = String::new();
    while let Some(card) = strategy.select(&mut cards) {
        print!(">{}", card.front());
        let mut stdin = io::stdin().lock();
        io::stdout().lock().flush().unwrap();
        stdin.read_line(&mut input).unwrap();
        print!(">>{}\n----------\n", card.back());
    }
    //for card in feeder {
    //}
    Ok(())
}
