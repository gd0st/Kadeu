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
    pub fn front(&self) -> &T {
        &self.front
    }
    pub fn back(&self) -> &U {
        &self.back
    }
}

#[derive(Deserialize, Clone)]
pub struct CardSet<T, U> {
    title: String,
    author: Option<String>,
    cards: Vec<Card<T, U>>,
}

impl<T, U> CardSet<T, U> {
    pub fn into_cards(self) -> Vec<Card<T, U>> {
        self.cards
    }
}

impl<'de, T, U> TryFrom<&'de str> for CardSet<T, U>
where
    T: Deserialize<'de>,
    U: Deserialize<'de>,
{
    type Error = serde_json::Error;

    fn try_from(value: &'de str) -> Result<Self, Self::Error> {
        serde_json::from_str(value)
    }
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
