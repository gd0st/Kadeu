use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;
use std::io;
use std::io::stdout;
use std::io::BufReader;
use std::path::Path;

use crate::game::engine::Engine;
use crate::strategies;
use crate::ui::CardSide;
use crate::{
    app,
    app::{CardBack, Deck},
};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};

pub struct App<T> {
    deck: Option<Deck<T>>,
    debugger: (Debugger<String>, bool),
}

type Card = app::Card<String, CardBack>;

impl<T> App<T> {
    pub fn new() -> Self {
        Self {
            deck: None,
            debugger: (Debugger::default(), false),
        }
    }

    pub fn with_debugger(mut self) -> Self {
        self.debugger.1 = true;
        self
    }
}

#[derive(Default)]
struct Debugger<T>(T);

impl<T> From<T> for Debugger<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T> Debugger<T>
where
    T: std::fmt::Display,
{
    fn widget(&self) -> impl Widget {
        let content = Text::from(self.0.to_string());
        let widget = Paragraph::new(content).block(Block::bordered().borders(Borders::ALL));
        widget
    }
    fn set_message(&mut self, message: T) {
        self.0 = message
    }
}

impl App<Card> {
    pub fn load<P: AsRef<Path>>(mut self, filepath: P) -> io::Result<Self> {
        let file = fs::OpenOptions::new().read(true).open(filepath)?;
        let reader = BufReader::new(file);
        let deck: Deck<Card> = serde_json::from_reader(reader)?;
        self.deck = Some(deck);
        Ok(self)
    }

    fn debugger_layout(area: Rect) -> std::rc::Rc<[Rect]> {
        let constraints = [Constraint::Ratio(9, 10), Constraint::Ratio(1, 10)];
        Layout::vertical(constraints).split(area)
    }

    pub fn set_deck(&mut self, deck: Deck<Card>) {
        self.deck = Some(deck);
    }

    pub fn dbg(&mut self, message: String) {
        self.debugger.0.set_message(message);
    }

    pub fn run(&mut self) -> io::Result<()> {
        let Some(deck) = &self.deck else {
            panic!("No deck loaded!")
        };

        // this is a fucking mess right now.
        let cards = deck.cards().into_iter().collect();
        let backend = CrosstermBackend::new(stdout());
        let mut terminal = Terminal::new(backend)?;
        //let mut slideshow = SlideShow::new();
        let mut current_card = CardSide::new("".to_string(), deck.title().to_string());
        current_card.reveal();
        let strategy = strategies::Random;
        let mut engine = Engine::new(cards);

        // let root = ui::Container::default();

        enable_raw_mode()?;

        stdout().execute(EnterAlternateScreen)?;

        // todo! just make this a hashmap instead.
        let _ = vec![
            ("Enter".to_string(), Action::Next),
            ("y".to_string(), Action::Restart),
            ("q".to_string(), Action::Quit),
        ];

        let mut hashed_pairs = HashMap::new();

        hashed_pairs.insert("y", Action::Restart);
        hashed_pairs.insert("Y", Action::Restart);
        hashed_pairs.insert("q", Action::Quit);
        hashed_pairs.insert("Enter", Action::Next);

        loop {
            let action = if let Some(key) = poll_keypress(50)? {
                hashed_pairs
                    .get(parse_press(key).as_str())
                    .unwrap_or(&Action::Continue)
                    .clone()
            } else {
                Action::Continue
            };

            terminal.draw(|f| {
                let mut area = f.area();

                if self.debugger.1 {
                    let areas = Self::debugger_layout(f.area());
                    f.render_widget(self.debugger.0.widget(), areas[1]);
                    area = areas[0];
                }
                f.render_widget(current_card.clone(), area);
            })?;

            match action {
                Action::Next => {
                    let cell = RefCell::new(current_card);
                    if !cell.borrow().is_revealed() {
                        cell.borrow_mut().reveal();
                        current_card = cell.into_inner();
                        continue;
                        //&cell.borrow_mut().reveal();
                    }
                    if let Some(card) = engine.next(&strategy) {
                        // need to figure out how to look at the next card front and back
                        // without losing reference to it.
                        // Requires having some sort of state saving system that can
                        // remember if the last card been revealed or not
                        // inolves using the pin features somehow but now sure how yet.
                        if cell.borrow().is_revealed() {
                            current_card =
                                CardSide::new(card.front().to_string(), card.back().to_string())
                                    .with_title(deck.title());

                            let debugger_message =
                                format!("Pushing card {}", card.front().to_string());
                            self.debugger.0.set_message(debugger_message);
                            continue;
                        }

                        current_card = cell.into_inner();

                        // current_card = current_card.reveal()
                    } else {
                        current_card =
                            CardSide::new("".to_string(), "Replay or Quit? [Y/q]".to_string())
                                .with_title(deck.title());
                        current_card.reveal();
                    }
                }
                Action::Restart => {
                    engine = Engine::new(deck.cards());
                    current_card = CardSide::new("".to_string(), deck.title().to_string());
                    current_card.reveal();
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

fn parse_press(key: KeyCode) -> String {
    match key {
        KeyCode::Enter => "Enter".to_string(),
        KeyCode::Char(c) => String::from(c),
        _ => "".to_string(),
    }
}
