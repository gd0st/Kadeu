pub mod app;
pub mod cli;
pub mod game;
pub mod io;
// pub mod tui;
pub mod ui;
//pub mod tui;
//mod store;
use crate::game::Kadeu;
use app::{Card, Deck};
use game::engine::Strategy;
use game::flashcard::{self};
use serde::Deserialize;
use serde_json;

use std::fmt::Display;

pub type Flashcard = flashcard::Flashcard<String, String>;

impl<T, U> Kadeu for Card<T, U>
where
    T: Display,
    U: Display,
{
    type Front = T;
    type Back = U;
    fn front(&self) -> &T {
        self.front()
    }

    fn back(&self) -> &U {
        self.back()
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

impl<'de, T> TryFrom<&'de str> for Deck<T>
where
    T: Deserialize<'de>,
{
    type Error = serde_json::Error;

    fn try_from(value: &'de str) -> Result<Self, Self::Error> {
        serde_json::from_str(value)
    }
}

mod strategies {
    use crate::{game::Kadeu, Strategy};
    use rand::{thread_rng, Rng};

    type strat<T: Kadeu> = fn(&[T]) -> T;
    pub struct Linear;
    impl<T> Strategy<T> for Linear {
        fn next(items: &mut Vec<T>) -> Option<T> {
            items.pop()
        }
    }

    pub struct Random;
    impl<T> Strategy<T> for Random {
        fn next(items: &mut Vec<T>) -> Option<T> {
            let num = thread_rng().gen_range(0..items.len());
            // can panic!
            Some(items.remove(num))
        }
    }
    #[cfg(test)]
    mod test {}
}
