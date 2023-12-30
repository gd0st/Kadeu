use clap::Parser;
use kadeu;
use kadeu::feeds::{Linear, Random};
use kadeu::game::feeder::Feeder;
use kadeu::game::{Kadeu, Score, Sequence};
use kadeu::model::{Card, CardBack, CardSet};
use std::collections::HashMap;
use std::fs;
use std::io::Result;
use std::io::{self, BufRead, Write};

#[derive(Parser, Debug)]
struct Config {
    #[clap(value_parser, default_value = "-")]
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
    let set: CardSet<String, CardBack> = CardSet::try_from(text.as_str())?;
    let feeder = Random::new(set.into_cards());
    let mut input = String::new();
    for card in feeder {
        print!(">{}", card.front());
        let mut stdin = io::stdin().lock();
        io::stdout().lock().flush().unwrap();
        stdin.read_line(&mut input).unwrap();
        print!(">>{}\n----------\n", card.back());
    }

    Ok(())
}
