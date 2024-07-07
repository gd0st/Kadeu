use std::fs;
use std::io;
use std::io::stdout;
use std::io::BufReader;
use std::path::Path;

use crate::game::Kadeu;
use crate::{
    app,
    app::{CardBack, Deck},
    strategy::{self, Strategy},
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

pub struct App<T> {
    deck: Option<Deck<T>>,
}

type Card = app::Card<String, CardBack>;

struct Game<T, V> {
    cards: Deck<T>,
    strategy: V,
}

impl<T> App<T> {
    pub fn new() -> Self {
        Self { deck: None }
    }
}

impl<T> App<T>
where
    T: DeserializeOwned + Kadeu,
{
    pub fn load<P: AsRef<Path>>(&mut self, filepath: P) -> io::Result<()> {
        let file = fs::OpenOptions::new().read(true).open(filepath)?;
        let reader = BufReader::new(file);
        let deck: Deck<T> = serde_json::from_reader(reader)?;
        self.deck = Some(deck);
        Ok(())
    }

    pub fn set_deck(&mut self, deck: Deck<T>) {
        self.deck = Some(deck);
    }

    pub fn run(&mut self) -> io::Result<()> {
        let Some(deck) = &self.deck else {
            panic!("No deck loaded!")
        };

        let mut stack = deck.make_stack();
        let backend = CrosstermBackend::new(stdout());
        let mut terminal = Terminal::new(backend)?;
        let strategy = strategy::Linear;
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        let mut _ui = Ui::new(deck.title());

        let pairs = vec![
            ("Enter", Action::Next),
            ("y", Action::Restart),
            ("q", Action::Quit),
        ];

        let mut output_buffer: Vec<String> = vec![];
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
                        if let Some(card) = stack.next(&strategy) {
                            let answer = format!("A: {}", card.display_back());
                            let question = format!("Q: {}", card.display_front());
                            // Pop question first, then pop the answer
                            output_buffer.push(answer);
                            output_buffer.push(question);
                        } else {
                            // Push End game notif if emtpy
                            output_buffer.push("No more card! Restart [Y/q]?".to_string())
                        }
                    }
                    if let Some(message) = output_buffer.pop() {
                        _ui.display(message)
                    }
                }
                Action::Restart => {
                    stack = deck.make_stack();
                    _ui = Ui::new(deck.title());
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
