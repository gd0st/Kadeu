use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::stdout;
use std::io::BufReader;
use std::ops::Deref;
use std::path::Path;

use crate::game::{
    engine::{Engine, Strategy},
    Kadeu,
};
use crate::strategies;
use crate::{
    app,
    app::{CardBack, Deck},
};

use crossterm::event::poll;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use serde::de::DeserializeOwned;
use serde::Deserialize;

mod views {
    use ratatui::{
        prelude::*,
        widgets::{self, *},
    };

    trait Window {
        fn widget(self) -> impl Widget;
    }
}

pub struct App<T> {
    deck: Option<Deck<T>>,
}

type Card = app::Card<String, CardBack>;

impl Card {
    fn prompt(&self) -> Paragraph {
        Paragraph::new(self.front().to_string())
    }
    fn answer(&self) -> Paragraph {
        Paragraph::new(self.back().to_string())
    }

    fn default(&self) -> impl Widget {
        let block = Block::new();
        let list = List::new([self.front().to_string(), self.back().to_string()]).block(block);
        list
    }
}

struct Game<T, V> {
    cards: Deck<T>,
    strategy: V,
}

impl<T> App<T> {
    pub fn new() -> Self {
        Self { deck: None }
    }
}

impl App<Card> {
    pub fn load<P: AsRef<Path>>(&mut self, filepath: P) -> io::Result<()> {
        let file = fs::OpenOptions::new().read(true).open(filepath)?;
        let reader = BufReader::new(file);
        let deck: Deck<Card> = serde_json::from_reader(reader)?;
        self.deck = Some(deck);
        Ok(())
    }

    pub fn set_deck(&mut self, deck: Deck<Card>) {
        self.deck = Some(deck);
    }

    pub fn run(&mut self) -> io::Result<()> {
        let Some(deck) = &self.deck else {
            panic!("No deck loaded!")
        };

        let cards = deck.cards();
        let backend = CrosstermBackend::new(stdout());
        let mut terminal = Terminal::new(backend)?;

        let strategy = strategies::Random;
        let mut engine = Engine::new(cards);

        let root = ui::Container::default();

        enable_raw_mode()?;

        stdout().execute(EnterAlternateScreen)?;
        let mut _ui = Ui::new(deck.title());

        // todo! just make this a hashmap instead.
        let pairs = vec![
            ("Enter", Action::Next),
            ("y", Action::Restart),
            ("q", Action::Quit),
        ];

        let mut output_buffer: Vec<Box<dyn Widget>> = vec![];
        loop {
            let key = poll_keypress(50)?;
            let action = if let Some(press) = key {
                if let Some(action) = get_action(press, pairs.clone()) {
                    action
                } else {
                    Action::Continue
                }
            } else {
                Action::Continue
            };

            draw(&mut terminal, &_ui)?;

            match action {
                Action::Next => {
                    if output_buffer.is_empty() {
                        if let Some(card) = engine.next(&strategy) {
                            let answer = format!("A: {}", card.display_back());
                            let question = format!("Q: {}", card.display_front());
                            // Pop question first, then pop the answer
                            output_buffer.push(Box::new(card.prompt()));
                            output_buffer.push(Box::new(card.answer()));
                        } else {
                            // Push End game notif if emtpy

                            output_buffer.push(Box::new(Paragraph::new(
                                "No more card! Restart [Y/q]?".to_string(),
                            )))
                        }
                    }
                    if let Some(widget) = output_buffer.pop() {
                        terminal.draw(|frame| frame.render_widget(widget.deref(), frame.size()));
                    }
                }
                Action::Restart => {
                    engine = Engine::new(deck.cards());
                }
                Action::Quit => break,
                _ => continue,
            };
        }
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
enum Action {
    Quit,
    Next,
    Restart,
    Continue,
}

fn handle_events() -> io::Result<Action> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => return Ok(Action::Quit),
                    KeyCode::Enter => return Ok(Action::Next),
                    _ => {}
                }
            }
        }
    }
    Ok(Action::Continue)
}

fn poll_keypress(tick: u64) -> io::Result<Option<KeyCode>> {
    if event::poll(std::time::Duration::from_millis(tick))? {
        if let Event::Key(event) = event::read()? {
            match event.kind {
                event::KeyEventKind::Press => return Ok(Some(event.code)),
                _ => return Ok(None),
            }
        }
    }
    Ok(None)
}

fn get_action(press: KeyCode, pairs: Vec<(&str, Action)>) -> Option<Action> {
    let press: Option<String> = match press {
        KeyCode::Enter => Some(String::from("Enter")),
        KeyCode::Char(c) => Some(String::from(c)),
        _ => None,
    };

    if let Some(key) = press {
        for (value, action) in pairs {
            if value == key {
                return Some(action);
            }
        }
    }
    None
}

mod ui {

    use ratatui::{
        backend::Backend,
        layout::{Columns, Constraint, Direction, Layout},
        prelude::CrosstermBackend,
        widgets::{Widget, WidgetRef},
        Terminal,
    };
    use std::io::Stdout;

    pub struct Container<T> {
        elements: Vec<Box<T>>,
    }
    impl<T> Default for Container<T> {
        fn default() -> Self {
            Self { elements: vec![] }
        }
    }
    impl<T> Container<T> {
        fn grid(direction: Direction, cols: u32) -> Layout {
            let columns: Vec<Constraint> = (0..cols).map(|_| Constraint::Ratio(1, cols)).collect();
            Layout::default().direction(direction).constraints(columns)
        }

        pub fn push(&mut self, widget: T) {
            self.elements.push(Box::new(widget))
        }
    }

    impl<T> WidgetRef for Container<T>
    where
        T: WidgetRef,
    {
        fn render_ref(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
            let cols = self.elements.len() as u32;
            let layout = Self::grid(Direction::Horizontal, cols).split(area);
            for (i, child) in self.elements.iter().enumerate() {
                child.render_ref(layout[i], buf)
            }
        }
    }
    struct Ui<B>
    where
        B: Backend,
    {
        terminal: Terminal<B>,
    }

    impl<B> Ui<B>
    where
        B: Backend,
    {
        pub fn render_container(&mut self, container: impl WidgetRef) {
            self.terminal
                .draw(|frame| container.render_ref(frame.size(), frame.buffer_mut()));
        }
    }
    //todo this can be generic
    impl TryFrom<CrosstermBackend<Stdout>> for Ui<CrosstermBackend<Stdout>> {
        type Error = std::io::Error;

        fn try_from(value: CrosstermBackend<Stdout>) -> std::io::Result<Self> {
            let terminal = Terminal::new(value)?;
            Ok(Self { terminal })
        }
    }

    #[cfg(test)]
    mod tests {
        use std::io::stdout;

        use ratatui::prelude::CrosstermBackend;

        use super::Ui;

        #[test]
        fn make_ui_crossterm() {
            let ui = Ui::try_from(CrosstermBackend::new(stdout()));
        }
    }
}

struct Ui {
    message: String,
}
impl Ui {
    fn new(init: &str) -> Self {
        Self {
            message: init.to_string(),
        }
    }

    fn display(&mut self, message: String) {
        self.message = message
    }
}

fn draw<B: Backend>(terminal: &mut Terminal<B>, ui: &Ui) -> io::Result<()> {
    let ui = |frame: &mut Frame| {
        frame.render_widget(
            Paragraph::new(ui.message.as_str())
                .block(Block::default().title("Hello World!").borders(Borders::ALL)),
            frame.size(),
        );
    };
    terminal.draw(ui)?;
    Ok(())
}

fn ui(frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new("Hello World!")
            .block(Block::default().title("Greeting").borders(Borders::ALL)),
        frame.size(),
    );
}
