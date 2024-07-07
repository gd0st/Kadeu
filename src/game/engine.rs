pub trait Strategy<T> {
    fn next(items: &mut Vec<T>) -> T;
}

pub struct Engine<T> {
    items: Vec<T>,
    // strategy: U,
}

impl<T> Engine<T> {
    pub fn new(items: Vec<T>) -> Self {
        // let strategy = U::new();
        Self { items }
    }

    fn next<U: Strategy<T>>(&mut self, _: U) -> Option<T> {
        if self.items.len() == 0 {
            return None;
        }
        Some(U::next(&mut self.items))
    }

    fn add(&mut self, item: T) {
        self.items.push(item)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use rand::seq::SliceRandom;
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

        if let Some(next) = engine.next(Linear) {
            assert_eq!(next, 1);
        }
    }

    #[test]
    fn build_engine_random() {
        let items = vec![1, 2];
        let items_ref = items.clone();
        let mut engine = Engine::new(items);
        if let Some(next) = engine.next(Random) {
            assert!(items_ref.contains(&next))
        }
    }
}
