pub mod game;
pub mod model;
mod store;

use crate::game::feeder::Feeder;
use crate::game::{Kadeu, Score};
use model::{Card, CardBack};
use serde::{Deserialize, Deserializer};
use serde_json;
use std::collections::HashMap;
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

pub mod feeds {
    use crate::game::feeder::Feeder;
    use crate::game::Kadeu;
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    use std::collections::HashMap;

    pub struct Linear<T>(Vec<T>);

    impl<T> Iterator for Linear<T> {
        type Item = T;
        fn next(&mut self) -> Option<Self::Item> {
            self.0.pop()
        }
    }

    impl<T> Feeder<T> for Linear<T> {
        fn new(items: Vec<T>) -> Self {
            Self(items)
        }
    }

    //TODO allow for seeding?
    pub struct Random<T> {
        items: Vec<T>,
    }

    impl<T> Feeder<T> for Random<T> {
        fn new(items: Vec<T>) -> Self {
            Self { items }
        }
    }

    impl<T> Iterator for Random<T> {
        type Item = T;
        fn next(&mut self) -> Option<Self::Item> {
            let mut rng = thread_rng();
            self.items.shuffle(&mut rng);
            self.items.pop()
        }
    }

    #[cfg(test)]
    mod test {
        use super::{Linear, Random};
        use crate::game::feeder::Feeder;
        use crate::game::Kadeu;
        use crate::model::Card;

        #[test]
        fn build_and_use_linear() {
            let cards = vec![Card::new("foo".to_string(), "bar".to_string())];
            let feeder = Linear::new(cards);

            for card in feeder {
                assert_eq!(card.front(), &"foo".to_string());
            }
        }
        #[test]
        fn build_and_use_random() {
            let cards = vec![
                Card::new("foo".to_string(), "bar".to_string()),
                Card::new("bizz".to_string(), "bazz".to_string()),
            ];
            let feeder = Random::new(cards);
            for card in feeder {
                match card.front().as_str() {
                    "foo" => assert!(true),
                    "bizz" => assert!(true),
                    _ => assert!(false),
                }
            }
        }
    }
}
