pub mod feeder {
    pub trait Feeder<T>
    where
        Self: Iterator<Item = T>,
    {
        fn new(items: Vec<T>) -> Self;
    }
}
pub trait Kadeu {
    fn prompt(&self) -> String {
        self.front()
    }
    fn front(&self) -> String;
    fn eval(&self, input: &String) -> Score;
}
pub enum Score {
    Hit,
    Miss,
}

impl<T> Kadeu for Progress<T>
where
    T: Kadeu,
{
    fn front(&self) -> String {
        self.item.prompt()
    }

    fn eval(&self, input: &String) -> Score {
        self.item.eval(input)
    }
}

pub struct Progress<T> {
    item: T,
    score: Option<Score>,
}

impl<T> Progress<T> {
    fn has_score(&self) -> bool {
        self.score.is_some()
    }

    fn score(&self) -> Option<&Score> {
        if let Some(score) = &self.score {
            Some(score)
        } else {
            None
        }
    }
}
pub trait Sequence<T>
where
    Self: Iterator<Item = T>,
{
    fn new(items: Vec<T>) -> Self;
}
