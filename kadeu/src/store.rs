
use crate::game::Kadeu;
use crate::game::Progress;
use crate::game::Score;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::Hash;
use std::io::Result;

trait ProgressStore {
    fn get_progress<T: Kadeu>(&self, card: &T) -> Progress<&T>;
    fn save_progress<T: Kadeu>(&mut self, card: &T, score: Score) -> Result<()>;
}
struct FileStore {
    root: String,
    progress: HashMap<String, Score>,
}

impl FileStore {
    fn new(root: String) -> Result<Self> {
        Ok(Self {
            root,
            progress: HashMap::new(),
        })
    }
}

impl ProgressStore for FileStore {
    fn save_progress<T: Kadeu>(&mut self, card: &T, score: Score) -> Result<()> {
        let mut s = DefaultHasher::new();
        let front = card.front();
        front.hash(&mut s);
        self.progress.insert(front, score);

        Ok(())
    }
    fn get_progress<T: Kadeu>(&self, card: &T) -> Progress<&T> {
        todo!()
    }
}
