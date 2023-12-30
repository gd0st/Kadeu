use serde::Deserialize;
use std::fmt::Display;

#[derive(Deserialize, Clone, Copy)]
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

#[derive(Deserialize, Clone)]
pub struct CardSet<T> {
    title: String,
    author: Option<String>,
    cards: Vec<T>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum CardBack {
    Word(String),
}

impl Display for CardBack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Word(answer) => write!(f, "{}", answer),
        }
    }
}
