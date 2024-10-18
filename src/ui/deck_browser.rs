use crate::game::Kadeu;
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
use super::{Action, Debugger, KadeuApp};

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

impl KadeuApp for DeckBrowser {
    fn handle_input(&mut self, input: Option<&Input>) -> std::io::Result<Action> {
        let Some(input) = input else {
            return Ok(Action::None);
        };
        let action = match input {
            Input::Up => {
                if self.index > 0 {
                    self.index -= 1;
                }
                Action::None
            }

            Input::Down => {
                if self.index < self.view().unwrap().len() - 1 {
                    self.index += 1;
                }
                Action::None
            }

            Input::Escape => {
                if self.root == self.parent {
                } else {
                    if let Some(parent) = self.root.parent() {
                        self.root = PathBuf::from(parent)
                    }
                }
                Action::Quit
            }
            Input::Select => {
                if self.current_path_is_file() {
                    Action::Exit
                } else {
                    self.traverse()?;
                    Action::None
                }
            }
            _ => Action::Continue,
        };
        Ok(action)
    }
    fn render<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> std::io::Result<()> {
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
        Ok(())
    }

    fn keymap(&self) -> KeyMap {
        let mut map = KeyMap::new();
        map.insert(KeyCode::Char('q'), Input::Quit);
        map.insert(KeyCode::Char('j'), Input::Down);
        map.insert(KeyCode::Char('k'), Input::Up);
        map.insert(KeyCode::Enter, Input::Select);
        map.insert(KeyCode::Backspace, Input::Escape);
        map
    }

    fn drop(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl DeckBrowser {
    pub fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) {}
    pub fn index(&self) -> usize {
        self.index
    }

    pub fn current_path_is_file(&self) -> bool {
        let Ok(paths) = self.paths() else {
            return false;
        };

        if paths.len() < 1 {
            return false;
        }
        let Some(path) = paths.get(self.index) else {
            panic!("the index is out of range")
        };

        !path.is_dir()
    }

    pub fn current_path(&self) -> Option<PathBuf> {
        let Ok(paths) = self.paths() else {
            return None;
        };

        if paths.len() < 1 {
            return None;
        }
        let Some(path) = paths.get(self.index) else {
            panic!("the index is out of range")
        };

        Some(path.to_owned())
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
    /// otherwise nothing happens
    pub fn traverse(&mut self) -> std::io::Result<()> {
        // TODO Fix this to not return an Action...
        // Actions will be done by handle_input instead. Bad code right now
        // still kinda a messy function
        if let Some(path) = self.paths()?.into_iter().nth(self.index) {
            if path.is_dir() {
                self.root = path.clone();
                self.index = 0;
            }
        }
        Ok(())
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
