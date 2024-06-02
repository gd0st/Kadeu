pub mod game;
pub mod model;
pub mod strategy;
pub mod tui;
//mod store;
use crate::game::{Kadeu, Score};
use game::feeder::Feeder;
use model::{Card, CardBack, CardSet};
use serde::Deserialize;
use serde_json;
use std::fmt::Display;
use std::io;

impl<T, U> Kadeu for Card<T, U>
where
    T: Display,
    U: Display,
{
    type Front = T;
    type Back = U;
    fn front(&self) -> &Self::Front {
        self.front()
    }

    fn back(&self) -> &Self::Back {
        self.back()
    }
    fn display_back(&self) -> String {
        self.back().to_string()
    }
    fn display_front(&self) -> String {
        self.front().to_string()
    }
}

impl<'de, T, U> TryFrom<&'de str> for Card<T, U>
where
    T: Deserialize<'de>,
    U: Deserialize<'de>,
{
    type Error = serde_json::Error;
    fn try_from(value: &'de str) -> Result<Self, Self::Error> {
        serde_json::from_str(value)
    }
}

impl<'de, T> TryFrom<&'de str> for CardSet<T>
where
    T: Deserialize<'de>,
{
    type Error = serde_json::Error;

    fn try_from(value: &'de str) -> Result<Self, Self::Error> {
        serde_json::from_str(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn make_card() {
        let card = Card::new("Hello".to_string(), CardBack::Word("World".to_string()));
        assert_eq!(card.front(), &"Hello".to_string())
    }

    #[test]
    fn hit_score() {
        let card = Card::new("Hello".to_string(), CardBack::Word("World".to_string()));
        let input = "World".to_string();
        let score = card.eval(&input);
        let hit = if let Score::Hit = score { true } else { false };
        assert!(hit)
    }
}
