pub trait Strategy<T> {
    fn next(items: &mut Vec<T>) -> T;
}

#[derive(Debug)]
pub struct Engine<T> {
    items: Vec<T>,
    // strategy: U,
}

impl<T> Engine<T> {
    pub fn new(items: Vec<T>) -> Self {
        // let strategy = U::new();
        Self { items }
    }

    pub fn next<U: Strategy<T>>(&mut self, _: &U) -> Option<T> {
        if self.items.len() == 0 {
            return None;
        }
        Some(U::next(&mut self.items))
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn add(&mut self, item: T) {
        self.items.push(item)
    }
}
