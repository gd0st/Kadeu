use game::{Kadeu, Score};
use model::{Card, CardBack};
use serde::{Deserialize, Deserializer};
use serde_json;
use std::fmt::Display;

impl<T> Kadeu for Card<T, CardBack>
where
    T: Display,
{
    fn front(&self) -> String {
        self.front().to_string()
    }

    fn eval(&self, input: &String) -> Score {
        match self.back() {
            CardBack::Word(answer) => {
                if answer == input {
                    return Score::Hit;
                }
            }
        }

        Score::Miss
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
        let score = card.eval(&input);
        let hit = if let Score::Hit = score { true } else { false };
        assert!(hit)
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
}
pub mod sequences {
    use super::game::Sequence;

    impl<T> Iterator for Linear<T> {
        type Item = T;
        fn next(&mut self) -> Option<T> {
            self.items.pop()
        }
    }

    pub struct Random<T> {
        items: Vec<T>,
    }

    pub struct Linear<T> {
        items: Vec<T>,
    }
    impl<T> Sequence<T> for Linear<T> {
        fn new(items: Vec<T>) -> Self {
            Self { items }
        }
    }
}
pub mod game {
    pub trait Kadeu {
        fn prompt(&self) -> String {
            self.front()
        }
        fn front(&self) -> String;
        fn eval(&self, input: &String) -> Score;
    }
    pub enum Score {
        Hit,
        Miss,
    }

    impl<T> Kadeu for Progress<T>
    where
        T: Kadeu,
    {
        fn front(&self) -> String {
            self.item.prompt()
        }

        fn eval(&self, input: &String) -> Score {
            self.item.eval(input)
        }
    }

    pub struct Progress<T> {
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
        Self: Iterator<Item = T>,
    {
        fn new(items: Vec<T>) -> Self;
    }
}

pub mod store {
    use crate::game::Kadeu;
    use crate::game::Progress;
    use crate::game::Score;
    use std::collections::hash_map::DefaultHasher;
    use std::collections::HashMap;
    use std::hash::Hash;
    use std::io::Result;

    trait ProgressStore {
        fn get_progress<T: Kadeu>(&self, card: &T) -> Progress<&T>;
        fn save_progress<T: Kadeu>(&mut self, card: &T, score: Score) -> Result<()>;
    }
    struct FileStore {
        root: String,
        progress: HashMap<String, Score>,
    }

    impl FileStore {
        fn new(root: String) -> Result<Self> {
            Ok(Self {
                root,
                progress: HashMap::new(),
            })
        }
    }

    impl ProgressStore for FileStore {
        fn save_progress<T: Kadeu>(&mut self, card: &T, score: Score) -> Result<()> {
            let mut s = DefaultHasher::new();
            let front = card.front();
            front.hash(&mut s);
            self.progress.insert(front, score);

            Ok(())
        }
        fn get_progress<T: Kadeu>(&self, card: &T) -> Progress<&T> {
            todo!()
        }
    }
}
