use std::io;
use std::io::stdout;

use kadeu::{
    model,
    model::{CardBack, CardSet},
    strategy::Strategy,
};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};

pub struct App<'a, B: Backend> {
    terminal: Terminal<B>,
    cards: Option<Vec<&'a Card>>,
}

type Card = model::Card<String, CardBack>;

struct Game<T, V> {
    cards: CardSet<T>,
    strategy: V,
}

impl<'a, B> App<'a, B>
where
    B: Backend,
{
    pub fn new(terminal: Terminal<B>) -> Self {
        Self {
            terminal,
            cards: None,
        }
    }

    pub fn add_set(&mut self, set: CardSet<Card<String, CardBack>>) {
        self.cards = Some(set.cards())
    }

    pub fn run(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout());
        let mut _ui = Ui::new();
        loop {
            draw(&mut self.terminal, &_ui)?;
            let action = handle_events()?;
            match action {
                Actions::Next => {
                    if let Some(card) = strategy.select(&mut cards) {
                        _ui.display(card.front().to_string())
                    } else {
                        self.cards = None
                    }
                }
                Actions::Quit => break,
                _ => continue,
            };
        }

        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }
}

impl<B> Drop for App<B>
where
    B: Backend,
{
    fn drop(&mut self) {
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
    }
}

enum Actions {
    Quit,
    Next,
    Continue,
}

fn handle_events() -> io::Result<Actions> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => return Ok(Actions::Quit),
                    KeyCode::Enter => return Ok(Actions::Next),
                    _ => {}
                }
            }
        }
    }
    Ok(Actions::Continue)
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

struct Ui {
    message: String,
}
impl Ui {
    fn new() -> Self {
        Self {
            message: "Greetings!".to_string(),
        }
    }

    fn display(&mut self, message: String) {
        self.message = message
    }
}
