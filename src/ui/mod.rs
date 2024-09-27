pub mod deck_browser;
pub mod inputs;
use std::path::PathBuf;

use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    text,
    widgets::{self, Block, Paragraph, Widget, WidgetRef},
};

use deck_browser::DeckBrowser;

use crate::app::Deck;

pub enum Action {
    Load(PathBuf),
    Quit,
    Continue,
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

fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}
