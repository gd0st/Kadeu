use rand::seq::SliceRandom;
use rand::thread_rng;

pub trait Strategy {
    fn shuffle<T>(&self, cards: Vec<T>) -> Vec<T> {
        cards
    }
}

pub struct Random;
impl Strategy for Random {
    fn shuffle<T>(&self, mut cards: Vec<T>) -> Vec<T> {
        let mut rng = thread_rng();
        cards.shuffle(&mut rng);
        cards
    }
}

pub struct Linear;
impl Strategy for Linear {}
