use rand::seq::SliceRandom;
use rand::thread_rng;

pub trait Strategy {
    fn select<T>(&self, items: &mut Vec<T>) -> Option<T> {
        items.pop()
    }
}

pub struct Random;
impl Strategy for Random {
    fn select<T>(&self, items: &mut Vec<T>) -> Option<T> {
        let mut rng = thread_rng();
        items.shuffle(&mut rng);
        items.pop()
    }
}

pub struct Linear;
impl Strategy for Linear {}
