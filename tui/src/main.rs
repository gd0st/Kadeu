use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use kadeu::{
    self,
    model::{Card, CardBack, CardSet},
    strategy::{self, Strategy},
};
use ratatui::{prelude::*, widgets::*};
use std::io::{self, stdout, Stdout};

enum Actions {
    Quit,
    Next,
    Continue,
}

fn main() -> io::Result<()> {
    let starting_deck = r#"
{
  "title": "foobar",
  "cards": [ { "front": "foo" , "back": "bar" }, {"front": "what the hell", "back": "is going on!"}]
}
"#;
    let set: CardSet<Card<String, CardBack>> = CardSet::try_from(starting_deck)?;
    let mut cards = set.cards();
    let stratgey = strategy::Linear;
    let mut numbers = (1..=3);
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    let mut _ui = Ui::new();
    loop {
        draw(&mut terminal, &_ui)?;
        let action = handle_events()?;
        match action {
            Actions::Next => {
                if let Some(card) = stratgey.select(&mut cards) {
                    _ui.display(card.front().to_string())
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
