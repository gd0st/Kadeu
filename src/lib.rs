pub mod app;
pub mod game;
pub mod tui;
pub mod ui;
//pub mod tui;
//mod store;
use crate::game::Kadeu;
use app::{Card, Deck};
use game::engine::Strategy;
use serde::Deserialize;
use serde_json;

use std::fmt::Display;

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
