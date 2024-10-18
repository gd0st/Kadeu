pub mod deck_browser;
pub mod inputs;
use std::path::PathBuf;

use crossterm::event::KeyCode;
use deck_browser::DeckBrowser;
use inputs::{Events, Input, KeyMap};
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    prelude::Backend,
    text,
    widgets::{Block, Paragraph, Widget, WidgetRef},
    Terminal,
};

pub trait KadeuApp {
    fn handle_input(&mut self, input: Option<&Input>) -> std::io::Result<Action>;
    fn render<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> std::io::Result<()>;
    // Allow for the app to cleanup anything before the end of it's lifecycle.
    fn drop(&mut self) -> std::io::Result<()> {
        Ok(())
    }
    fn keymap(&self) -> KeyMap {
        [(KeyCode::Char('q'), Input::Quit)].into()
    }
}

pub struct AppHandler<B>
where
    B: Backend,
{
    terminal: Terminal<B>,
    events: Events,
    tick: u64,
}

impl<B> AppHandler<B>
where
    B: Backend,
{
    pub fn set_keymap(&mut self, keymap: KeyMap) {
        self.events = Events::from(keymap)
    }

    pub fn run(&mut self, app: &mut impl KadeuApp) -> std::io::Result<Action> {
        self.events = Events::from(app.keymap());
        loop {
            let input = self.events.poll(self.tick)?;

            if let Some(Input::Quit) = input {
                return Ok(Action::Quit);
            }
            let action = app.handle_input(input)?;

            if let Action::None = action {
                app.render(&mut self.terminal)?;
            } else {
                return Ok(action);
            }
        }
    }
}

impl<B> From<Terminal<B>> for AppHandler<B>
where
    B: Backend,
{
    fn from(terminal: Terminal<B>) -> Self {
        Self {
            terminal,
            events: Events::default(),
            tick: 64,
        }
    }
}

// These are actually instructions, not actions lol
pub enum Action {
    Exit,
    Load(PathBuf),
    Quit,
    Continue,
    None,
}

trait Debugger {
    fn text(&self) -> text::Text;
}

impl WidgetRef for dyn Debugger {
    fn render_ref(&self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        Paragraph::new(self.text()).render_ref(area, buf);
    }
}

#[derive(Clone)]
pub struct CardSide {
    deck_title: Option<String>,
    front: String,
    back: String,
    revealed: bool,
}

impl CardSide {
    pub fn new(front: String, back: String) -> Self {
        Self {
            deck_title: None,
            front: front,
            back: back,
            revealed: false,
        }
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.deck_title = Some(title.to_string());
        self
    }

    pub fn reveal(&mut self) {
        self.revealed = true;
    }

    pub fn is_revealed(&self) -> bool {
        self.revealed
    }
}

impl Widget for CardSide {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let content = if self.is_revealed() {
            self.back
        } else {
            self.front
        };
        let mut text = Text::new(&content).bordered(&[]).centered();
        if let Some(title) = self.deck_title.as_ref() {
            text = text.with_border_title(title);
        }
        text.render(area, buf)
    }
}

impl Widget for Text {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let text = text::Text::from(self.text);
        let center_area = if self.centered {
            center(
                area,
                Constraint::Length(text.width() as u16),
                Constraint::Length(1),
            )
        } else {
            area
        };
        text.render_ref(center_area, buf);

        if self.bordered {
            let block = if let Some(title) = &self.border_title {
                Block::bordered().title(title.to_string())
            } else {
                Block::bordered()
            };

            block.render_ref(area, buf);
        }
    }
}

#[derive(Default)]
pub struct Text {
    text: String,
    centered: bool,
    bordered: bool,
    border_title: Option<String>,
    border_styles: Vec<String>,
}

impl Text {
    pub fn new(text: &str) -> Self {
        let mut this = Self::default();
        this.text = text.to_string();
        this
    }
    pub fn centered(mut self) -> Self {
        self.centered = true;
        self
    }

    pub fn with_border_title(mut self, title: &str) -> Self {
        self.border_title = Some(title.to_string());
        self
    }

    pub fn bordered(mut self, styles: &[String]) -> Self {
        self.bordered = true;
        self.border_styles = Vec::from(styles);
        self
    }
}

pub fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}
