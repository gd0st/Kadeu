use serde::Deserialize;
use std::fmt::Display;

use crate::strategy::{self, Strategy};

#[derive(Deserialize, Clone, Copy, Debug)]
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

impl<T> CardSet<T> {
    pub fn title(&self) -> &str {
        self.title.as_str()
    }
}

#[derive(Clone, Debug)]
pub struct CardStack<'a, T> {
    stack: Vec<&'a T>,
}

impl<'a, T> CardStack<'a, T> {
    pub fn new(cards: Vec<&'a T>) -> Self {
        Self { stack: cards }
    }

    pub fn next(&mut self, strategy: &impl Strategy) -> Option<&'a T> {
        if !self.is_empty() {
            self.stack = strategy.shuffle(self.stack.clone());
            self.stack.pop()
        } else {
            None
        }
    }
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

impl<T> CardSet<T> {
    pub fn cards(&self) -> Vec<&T> {
        self.cards.iter().collect()
    }

    pub fn make_stack(&self) -> CardStack<T> {
        CardStack::new(self.cards.iter().collect())
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
