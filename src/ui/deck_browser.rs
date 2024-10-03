use crate::ui::inputs::Input;
use crossterm::event::KeyCode;
use ratatui::prelude::{Backend, CrosstermBackend};
use ratatui::style::Stylize;
use ratatui::text::Text;
use ratatui::widgets::{Block, List, ListItem, ListState, StatefulWidgetRef};
use ratatui::Terminal;
use std::cmp::max;
use std::fs;
use std::io::Stdout;
use std::path::{Path, PathBuf};

use super::inputs::{EventListener, Events, KeyMap};
use super::{Action, Debugger};

pub struct DeckBrowser {
    parent: PathBuf,
    root: PathBuf,
    index: usize,
}

impl From<PathBuf> for DeckBrowser {
    fn from(root: PathBuf) -> Self {
        Self {
            parent: root.clone(),
            root,
            index: 0,
        }
    }
}
impl Debugger for DeckBrowser {
    fn text(&self) -> ratatui::text::Text {
        let err: String = if let Err(e) = self.view() {
            e.to_string()
        } else {
            "OK!".to_string()
        };
        let text = format!(
            "deckbrowser:{},{},{} ",
            self.index,
            self.root.to_string_lossy(),
            err
        );
        Text::from(text)
    }
}

pub trait KadeuApp<B>
where
    B: Backend,
{
    fn render(&mut self, terminal: &mut Terminal<B>, events: &Events) -> std::io::Result<Action>;
}

impl<B> KadeuApp<B> for DeckBrowser
where
    B: Backend,
{
    fn render(&mut self, terminal: &mut Terminal<B>, events: &Events) -> std::io::Result<Action> {
        // TODO allow style sheets to be passed in.
        let mut keymap = KeyMap::new();
        keymap.insert(KeyCode::Char('q'), Input::Quit);
        keymap.insert(KeyCode::Char('j'), Input::Down);
        keymap.insert(KeyCode::Char('k'), Input::Up);
        keymap.insert(KeyCode::Backspace, Input::Escape);
        keymap.insert(KeyCode::Enter, Input::Select);
        // TODO allow the tick rate to be adjusted

        let action = if let Some(input) = events.poll(64)? {
            match input {
                Input::Quit => Action::Quit,
                anything => self.input(anything)?,
            }
        } else {
            Action::Continue
        };
        let view = self.view()?;

        let items = view.into_iter().map(|item| Text::from(item).white());
        let block = Block::default().title(self.fullpath().to_string_lossy().to_string());
        let list = List::new(items)
            .on_black()
            .white()
            .highlight_symbol("> ")
            .block(block);
        let mut state = ListState::default();
        state.select(Some(self.index));
        terminal.draw(|frame| {
            frame.render_stateful_widget(list, frame.area(), &mut state);
        })?;
        Ok(action)
    }
}

impl DeckBrowser {
    pub fn run(termina: &mut Terminal<CrosstermBackend<Stdout>>) {}
    pub fn index(&self) -> usize {
        self.index
    }

    pub fn paths(&self) -> std::io::Result<Vec<PathBuf>> {
        let mut paths = vec![];
        for entry in fs::read_dir(&self.root)? {
            let entry = entry?;
            let path = entry.path();
            let Some(filename) = path.file_name() else {
                continue;
            };

            if !filename.to_string_lossy().starts_with(".") {
                paths.push(path);
            }
        }
        Ok(paths)
    }
    // TODO just return stylized text instead
    pub fn view(&self) -> std::io::Result<Vec<String>> {
        let paths = self
            .paths()?
            .iter()
            .map(|path| {
                format!(
                    "{}{}",
                    &path.to_string_lossy().to_string(),
                    if path.is_dir() { ">" } else { "" }
                )
            })
            .collect();
        Ok(paths)
    }

    pub fn fullpath(&self) -> PathBuf {
        let mut path = self.parent.clone();
        if path != self.root {
            path.push(self.root.clone());
        }
        path
    }

    /// Will change it's state to browse the child folder
    /// if the self.index is discovered to be on a path
    /// otherwise if it's a file it will send an Action back with the PathBuf of the selection
    /// Or it will signal to do nothing.
    pub fn traverse(&mut self) -> std::io::Result<Action> {
        // still kinda a messy function
        if let Some(path) = self.paths()?.into_iter().nth(self.index) {
            if path.is_dir() {
                self.root = path.clone();
                self.index = 0;
                return Ok(Action::Continue);
            } else if path.is_file() {
                return Ok(Action::Load(path));
            }
        }
        Ok(Action::Continue)
    }

    pub fn input(&mut self, input: &Input) -> std::io::Result<Action> {
        let action = match input {
            Input::Up => {
                if self.index > 0 {
                    self.index -= 1;
                }
                Action::Continue
            }

            Input::Down => {
                if self.index < self.view().unwrap().len() - 1 {
                    self.index += 1;
                }
                Action::Continue
            }

            Input::Escape => {
                if self.root == self.parent {
                } else {
                    if let Some(parent) = self.root.parent() {
                        self.root = PathBuf::from(parent)
                    }
                }
                Action::Continue
            }
            Input::Select => self.traverse()?,
            _ => Action::Continue,
        };
        Ok(action)
    }
}

impl EventListener for DeckBrowser {
    fn on_event(&mut self, input: &Input) {
        match input {
            Input::Up => {
                if self.index > 0 {
                    self.index -= 1;
                }
            }

            Input::Down => self.index = max(self.view().unwrap_or_default().len(), self.index + 1),
            _ => {}
        }
    }
}

// ummm.... okay?
impl<'a> Into<Option<(List<'a>, ListState)>> for &'a DeckBrowser {
    fn into(self) -> Option<(List<'a>, ListState)> {
        if let Ok(view) = self.view() {
            let items = view.into_iter().map(|item| Text::from(item));
            let mut state = ListState::default();
            state.select(Some(self.index));
            let list = List::new(items);
            Some((list, state))
        } else {
            None
        }
    }
}
