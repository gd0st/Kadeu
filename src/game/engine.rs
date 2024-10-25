use super::Kadeu;
use ratatui::text::Text;
use std::{collections::VecDeque, fmt::Display};

type StrategyFunction<T> = fn(&mut VecDeque<T>) -> Option<T>;

mod strategies {
    use rand::{thread_rng, Rng};
    use std::collections::VecDeque;

    pub trait Strat<T> {
        fn next(items: &mut VecDeque<T>) -> Option<T>;
        fn sequence(&self, items: &mut VecDeque<T>) -> impl Iterator<Item = T>;
    }

    //init Random generator
    //allow a seed to be set
    pub struct Random {
        seed: usize,
    }

    impl<T> Strat<T> for Random {
        fn next(items: &mut VecDeque<T>) -> Option<T> {
            let mut rng = thread_rng();
            None
        }

        fn sequence(&self, items: &mut VecDeque<T>) -> impl Iterator<Item = T> {
            vec![].into_iter()
        }
    }

    pub enum Strategy {
        Linear,
        Random,
    }
    pub fn linear<T>(items: &mut VecDeque<T>) -> Option<T> {
        items.pop_front()
    }

    pub fn random<T>(items: &mut VecDeque<T>) -> Option<T> {
        if items.is_empty() {
            return None;
        }
        let index = thread_rng().gen_range(0..items.len());
        items.remove(index)
    }
}

#[derive(Debug)]
struct Pin<T> {
    item: T,
    pinned: bool,
}

// This looks horendous
impl<'a, K, T, U> Into<Text<'a>> for &Pin<K>
where
    K: Kadeu<Front = T, Back = U>,
    T: Display,
    U: Display,
{
    fn into(self) -> Text<'a> {
        if self.pinned {
            self.item.front().to_string()
        } else {
            self.item.back().to_string()
        }
        .into()
    }
}

impl<T> From<T> for Pin<T> {
    fn from(item: T) -> Self {
        Self { item, pinned: true }
    }
}

impl<T> Pin<T> {
    fn unpin(&mut self) {
        self.pinned = false;
    }
    fn get_mut(&mut self) -> &mut T {
        &mut self.item
    }
}

pub trait Strategy<T> {
    fn next(items: &mut Vec<T>) -> Option<T>;
}

#[derive(Debug)]
pub struct Engine<T> {
    items: VecDeque<T>,
    current: Option<Pin<T>>,
    strategy: StrategyFunction<T>,
}

impl<T> Engine<T> {
    pub fn new(items: VecDeque<T>, strat: StrategyFunction<T>) -> Self {
        // let strategy = U::new();
        Self {
            items,
            current: None,
            strategy: strat,
        }
    }

    pub fn next(&mut self) {
        if let Some(item) = (self.strategy)(&mut self.items) {
            self.current = Some(item.into());
        };
        // self.current()
    }

    pub fn unpin_current(&mut self) {
        if let Some(current) = self.current.as_mut() {
            current.unpin();
        }
    }

    fn current(&self) -> Option<&Pin<T>> {
        if let Some(item) = &self.current {
            Some(item)
        } else {
            None
        }
    }

    pub fn current_mut(&mut self) -> Option<&mut T> {
        if let Some(item) = self.current.as_mut() {
            Some(item.get_mut())
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn add(&mut self, item: T) {
        self.items.push_front(item);
    }
}

pub mod ui {
    use std::{collections::VecDeque, fmt::Display};

    use crossterm::event::KeyCode;
    use ratatui::{text::Text, widgets::Block};

    use super::{strategies, Engine};
    use crate::{
        app::Deck,
        game::Kadeu,
        ui::{
            center,
            inputs::{Input, KeyMap},
            Exit, KadeuApp,
        },
    };

    pub struct FlashcardApp<T> {
        show_title: bool,
        show_end: bool,
        engine: Engine<T>,
        title: String,
    }

    impl<T> From<Deck<T>> for FlashcardApp<T> {
        fn from(deck: Deck<T>) -> Self {
            Self {
                show_title: true,
                show_end: false,
                title: deck.title().to_string(),
                // TODO figure out where this strategy is sourced from??
                engine: Engine::new(VecDeque::from(deck.into_cards()), strategies::linear),
            }
        }
    }

    impl<T, U, V> KadeuApp for FlashcardApp<T>
    where
        T: Kadeu<Front = U, Back = V>,
        U: Display,
        V: Display,
    {
        fn keymap(&self) -> KeyMap {
            let mut keymap = KeyMap::new();
            keymap.insert(KeyCode::Char('q'), Input::Escape);
            keymap.insert(KeyCode::Enter, Input::Continue);
            keymap.insert(KeyCode::Esc, Input::Escape);
            keymap
        }
        fn render<B: ratatui::prelude::Backend>(
            &mut self,
            terminal: &mut ratatui::Terminal<B>,
        ) -> std::io::Result<()> {
            if self.show_title {
                let text = Text::from(self.title.to_string());
                let block = Block::bordered();
                terminal.draw(|frame| {
                    let area = center(
                        frame.area(),
                        ratatui::layout::Constraint::Length(text.width() as u16),
                        ratatui::layout::Constraint::Length(1),
                    );
                    frame.render_widget(block, frame.area());
                    frame.render_widget(text, area);
                })?;
                return Ok(());
            }

            let Some(item) = self.engine.current() else {
                println!("here!");
                // Draw End Splash
                return Ok(());
            };

            let mut text: Text = item.into();
            text = text.centered();
            let block = Block::bordered().title(self.title.to_string());
            terminal.draw(|frame| {
                let area = center(
                    frame.area(),
                    ratatui::layout::Constraint::Length(text.width() as u16),
                    ratatui::layout::Constraint::Length(1),
                );
                frame.render_widget(block, frame.area());
                frame.render_widget(text, area);
            })?;

            Ok(())
        }
        fn handle_input(&mut self, input: Option<&Input>) -> std::io::Result<Exit> {
            if let Some(Input::Continue) = input {
                match self.engine.current() {
                    Some(current) => {
                        if current.pinned {
                            self.engine.unpin_current();
                        } else {
                            self.engine.next();
                        }
                    }
                    None => {
                        if self.show_title {
                            self.show_title = false;
                        }
                        self.engine.next();

                        if self.engine.current().is_none() {
                            self.show_end = true;
                        }
                    }
                }
            }

            if let Some(Input::Escape) = input {
                return Ok(Exit::Drop);
            }

            if let Some(Input::Quit) = input {
                return Ok(Exit::Quit);
            }
            Ok(Exit::None)
        }
    }
}
