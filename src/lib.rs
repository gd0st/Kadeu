pub mod app;
pub mod game;
pub mod tui;
mod ui;
//pub mod tui;
//mod store;
use crate::game::{Kadeu, Score};
use app::{Card, CardBack, Deck};
use game::engine::Strategy;
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
    use crate::Strategy;
    use rand::{thread_rng, Rng};
    pub struct Linear;
    impl<T> Strategy<T> for Linear {
        fn next(items: &mut Vec<T>) -> T {
            items.remove(0)
        }
    }

    pub struct Random;
    impl<T> Strategy<T> for Random {
        fn next(items: &mut Vec<T>) -> T {
            let num = thread_rng().gen_range(0..items.len());
            items.remove(num)
        }
    }
    #[cfg(test)]
    mod test {

        use super::*;
        use crate::game::engine::Engine;
        use rand::{thread_rng, Rng};

        struct Linear;
        impl<T> Strategy<T> for Linear {
            fn next(items: &mut Vec<T>) -> T {
                items.remove(0)
            }
        }

        struct Random;
        impl<T> Strategy<T> for Random {
            fn next(items: &mut Vec<T>) -> T {
                let num = thread_rng().gen_range(0..items.len());
                items.remove(num)
            }
        }

        #[test]
        fn build_engine_linear() {
            let items = vec![1, 2, 3];
            let mut engine = Engine::new(items);

            if let Some(next) = engine.next(&Linear) {
                assert_eq!(next, 1);
            }
        }

        #[test]
        fn build_engine_random() {
            let items = vec![1, 2];
            let items_ref = items.clone();
            let mut engine = Engine::new(items);
            if let Some(next) = engine.next(&Random) {
                assert!(items_ref.contains(&next))
            }
        }
    }
}

struct Pin<T>(T, bool);

impl<T> From<T> for Pin<T> {
    fn from(value: T) -> Self {
        Pin(value, false)
    }
}

impl<T> Pin<T> {
    pub fn new(value: T) -> Self {
        Self::from(value)
    }

    pub fn unpin(&mut self) {
        self.1 = true;
    }

    pub fn pinned(&self) -> bool {
        self.1
    }

    pub fn get_ref(&self) -> &T {
        &self.0
    }

    pub fn get_ref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn make_card() {}

    #[test]
    fn hit_score() {}
}
