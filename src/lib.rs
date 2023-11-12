use game::{Kadeu, Score};
use model::{Card, CardBack};
use serde::Deserialize;
use serde_json;
use std::fmt::Display;

impl Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Score::Hit => write!(f, "{}", "hit"),
            Score::Miss => write!(f, "{}", "miss"),
        }
    }
}

impl<T> Kadeu for Card<T, CardBack>
where
    T: Display,
{
    fn prompt(&self) -> String {
        self.front().to_string()
    }

    fn eval(&self, input: &String) -> Option<Score> {
        match self.back() {
            CardBack::Word(answer) => {
                if answer == input {
                    return Some(Score::Hit);
                }
            }
        }

        Some(Score::Miss)
    }
}

pub fn from_str<'de, T>(text: &'de str) -> serde_json::Result<T>
where
    T: Deserialize<'de>,
{
    serde_json::from_str(text)
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
        if let Some(score) = card.eval(&input) {
            let hit = if let Score::Hit = score { true } else { false };
            assert!(hit)
        };
    }
}

pub mod model {
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

    impl From<String> for CardBack {
        fn from(value: String) -> Self {
            Self::Word(value)
        }
    }
}
pub mod sequences {
    use super::game::Sequence;
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    use std::vec;

    struct GenericSequence<T> {
        items: Vec<T>,
        sequence: vec::IntoIter<T>,
    }

    impl<T> GenericSequence<T> {
        fn new(mut items: Vec<T>, strategy: fn(Vec<T>) -> Vec<T>) -> Linear<T> {
            Linear::new(strategy(items))
        }
    }

    impl<T> IntoIterator for GenericSequence<T> {
        type Item = T;
        type IntoIter = vec::IntoIter<T>;
        fn into_iter(self) -> Self::IntoIter {
            self.sequence
        }
    }

    impl<T> IntoIterator for Linear<T> {
        type Item = T;
        type IntoIter = std::vec::IntoIter<T>;
        fn into_iter(self) -> Self::IntoIter {
            self.items.into_iter()
        }
    }

    pub struct Linear<T> {
        items: Vec<T>,
    }
    impl<T> Sequence<T> for Linear<T> {
        fn new(items: Vec<T>) -> Self {
            let mut sequence = Self { items };
            sequence.items.reverse();
            sequence
        }
    }

    pub struct Random<T> {
        items: Vec<T>,
    }

    impl<T> IntoIterator for Random<T> {
        type Item = T;
        type IntoIter = std::vec::IntoIter<T>;
        fn into_iter(self) -> Self::IntoIter {
            self.items.into_iter()
        }
    }

    impl<T> Sequence<T> for Random<T> {
        fn new(items: Vec<T>) -> Self {
            let mut sequence = Self { items };
            sequence.items.shuffle(&mut thread_rng());
            sequence
        }
    }
    #[cfg(test)]
    mod test {
        use crate::model::Card;
        use crate::model::CardBack;

        #[test]
        fn make_generic() {
            let card = Card::new("Hello".to_string(), CardBack::Word("World".to_string()));
            let ocard = Card::new("foobar".to_string(), CardBack::Word("bizzbazz".to_string()));
        }
    }
}
pub mod game {
    pub trait Kadeu {
        fn prompt(&self) -> String;
        fn eval(&self, input: &String) -> Option<Score>;
    }
    pub enum Score {
        Hit,
        Miss,
    }

    impl<T> Kadeu for Progress<T>
    where
        T: Kadeu,
    {
        fn prompt(&self) -> String {
            self.item.prompt()
        }

        fn eval(&self, input: &String) -> Option<Score> {
            self.item.eval(input)
        }
    }

    struct Progress<T> {
        item: T,
        score: Option<Score>,
    }

    impl<T> Progress<T> {
        fn has_score(&self) -> bool {
            self.score.is_some()
        }

        fn score(&self) -> Option<&Score> {
            if let Some(score) = &self.score {
                Some(score)
            } else {
                None
            }
        }
    }
    pub trait Sequence<T>
    where
        Self: IntoIterator,
    {
        fn new(items: Vec<T>) -> Self;
    }
}
