use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::Kadeu;

#[derive(Deserialize, Serialize)]
pub struct Flashcard<T, U> {
    front: T,
    back: U,
}

impl<T, U> Kadeu for Flashcard<T, U>
where
    T: Display,
    U: Display,
{
    type Front = T;
    type Back = U;

    fn front(&self) -> &Self::Front {
        &self.front
    }

    fn back(&self) -> &Self::Back {
        &self.back
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
enum CardBack {
    Word(String),
    Number(isize),
}

impl Display for CardBack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Word(word) => write!(f, "{}", word),
            Self::Number(num) => write!(f, "{}", num),
        }
    }
}
