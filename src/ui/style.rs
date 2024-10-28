use core::fmt;

use ratatui::{
    style::{Color, Style, Stylize},
    text::Text,
    widgets::{Block, List, ListItem},
};

#[derive(Clone, Debug)]
pub struct AppStyle {
    background_color: Color,
    border_color: Color,
    text_color: Color,
}

impl Default for AppStyle {
    fn default() -> Self {
        Self {
            background_color: Color::Black,
            // Still needs work
            border_color: Color::White,
            text_color: Color::White,
        }
    }
}

impl AppStyle {
    pub fn bg(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }
    pub fn block(&self) -> Block {
        Block::new()
            .bg(self.background_color)
            .border_style(Style::new().fg(self.border_color))
    }

    fn text<T: fmt::Display>(&self, item: T) -> Text<'_> {
        Text::from(item.to_string()).style(Style::new().fg(self.text_color))
    }

    pub fn list<U: fmt::Display, T: IntoIterator<Item = U>>(&self, items: T) -> List {
        let items = items.into_iter().map(|item| self.text(item));
        List::new(items).block(self.block())
    }
}
