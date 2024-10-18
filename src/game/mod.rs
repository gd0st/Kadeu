use std::fmt::Display;

pub mod engine;
pub mod flashcard;
pub trait Kadeu {
    type Front;
    type Back;
    fn front(&self) -> &Self::Front;
    fn back(&self) -> &Self::Back;
}

impl<T, U> Kadeu for (T, U)
where
    T: Display,
    U: Display,
{
    type Front = T;
    type Back = U;
    fn back(&self) -> &Self::Back {
        &self.1
    }

    fn front(&self) -> &Self::Front {
        &self.0
    }
}

pub enum Score {
    Hit,
    Miss,
}

impl Score {
    pub fn to_string(&self) -> String {
        String::from(match self {
            Self::Hit => "hit",
            Self::Miss => "miss",
        })
    }
}

pub struct Progress<T> {
    item: T,
    score: Option<Score>,
}

impl<T> Progress<T> {
    pub fn set_score(&mut self, score: Score) {
        self.score = Some(score)
    }

    fn score(&self) -> Option<&Score> {
        if let Some(score) = &self.score {
            Some(score)
        } else {
            None
        }
    }
}
