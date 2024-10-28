pub mod deck_browser;
pub mod inputs;
pub mod style;
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
use style::AppStyle;

pub trait KadeuApp {
    fn handle_input(&mut self, input: Option<&Input>) -> std::io::Result<Exit>;
    fn render<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
        style: &style::AppStyle,
    ) -> std::io::Result<()>;
    // Allow for the app to cleanup anything before the end of it's lifecycle.
    fn drop(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    fn style(&self) -> AppStyle {
        AppStyle::default()
    }

    /// Enables the universal keymap for a given application.
    /// This can be useful if you don't want the main app runner
    /// To perform a universal action such as quitting.
    fn disable_universal_keymap(&self) -> bool {
        false
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

pub fn run<B: Backend, K: KadeuApp>(
    terminal: &mut Terminal<B>,
    app: &mut K,
    events: Events,
    tickrate: u64,
) -> std::io::Result<Exit> {
    let style = app.style();
    loop {
        let input = events.poll(tickrate)?;

        if !app.disable_universal_keymap() {
            if let Some(Input::Quit) = input {
                return Ok(Exit::Quit);
            }
        }

        let action = app.handle_input(input)?;

        if let Exit::None = action {
            app.render(terminal, &style)?;
        } else {
            return Ok(action);
        }
    }
}

impl<B> AppHandler<B>
where
    B: Backend,
{
    pub fn set_keymap(&mut self, keymap: KeyMap) {
        self.events = Events::from(keymap)
    }

    pub fn run(&mut self, app: &mut impl KadeuApp) -> std::io::Result<Exit> {
        self.events = Events::from(app.keymap());
        let style = AppStyle::default();
        loop {
            let input = self.events.poll(self.tick)?;

            if !app.disable_universal_keymap() {
                if let Some(Input::Quit) = input {
                    return Ok(Exit::Quit);
                }
            }

            let action = app.handle_input(input)?;

            if let Exit::None = action {
                app.render(&mut self.terminal, &style)?;
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

/// Communicates to the run function if the app would like to exit
/// and if so in what state it would like to exit (drop and allow the main app to handle the next step?).
/// None is used to communicate that nothing should be done and the next frame can be rendered.
pub enum Exit {
    Drop,
    Quit,
    None,
}

// TODO Debugger needs to be reworked
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

pub fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}
