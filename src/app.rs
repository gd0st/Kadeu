use serde::{Deserialize, Serialize};
use std::fmt::Display;

//use crate::strategy::{self, Strategy};

mod io {
    use std::io::Write;

    use super::Deck;
    use serde::Serialize;
    use serde_json;
    pub enum Serialization {
        Json,
        Yaml,
    }
    impl Serialization {
        pub fn write<T: Serialize>(
            &self,
            deck: &Deck<T>,
            writer: impl Write,
        ) -> std::io::Result<()> {
            let res = match self {
                Self::Json => Self::write_json(deck, writer),
                Self::Yaml => Self::write_yaml(deck, writer),
            };
			res
        }

        fn write_json<T: Serialize>(deck: &Deck<T>, writer: impl Write) -> std::io::Result<()> {
            let res = serde_json::to_writer(writer, deck)
            match res {
                Err(e) => Err(std::io::Error::other(e)),
                _ => Ok(()),
            }
        }

		fn write_yaml<T: Serialize>(deck: &Deck<T>, writer: impl Write) -> std::io::Result<()> {
            let res = serde_json::to_writer(writer, deck)
            match res {
                Err(e) => Err(std::io::Error::other(e)),
                _ => Ok(()),
            }
		}
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Card<T, U> {
    front: T,
    back: U,
}
impl<T, U> Card<T, U> {
    pub fn new(front: T, back: U) -> Self {
        Self { front, back }
    }
    pub fn back(&self) -> &U {
        &self.back
    }

    pub fn front(&self) -> &T {
        &self.front
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Deck<T> {
    title: String,
    author: Option<String>,
    cards: Vec<T>,
}


impl<T> Deck<T> {
    pub fn title(&self) -> &str {
        self.title.as_str()
    }
}

impl<T> Deck<T> {
    pub fn cards(&self) -> Vec<&T> {
        self.cards.iter().collect()
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum CardBack {
    Word(String),
    Number(usize),
}

impl Display for CardBack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Word(answer) => write!(f, "{}", answer),
            Self::Number(answer) => write!(f, "{}", answer),
        }
    }
}
