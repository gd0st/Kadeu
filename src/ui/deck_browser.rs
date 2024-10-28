use crate::app::{Card, Deck};
use crate::game::Kadeu;
use crate::io::FileType;
use crate::ui::inputs::Input;
use crossterm::event::KeyCode;
use ratatui::prelude::Backend;
use ratatui::style::Color;
use ratatui::text::Text;
use ratatui::widgets::ListState;
use ratatui::Terminal;
use serde::de::DeserializeOwned;
use std::ffi::OsString;
use std::fs;
use std::mem::swap;
use std::path::PathBuf;

use super::inputs::KeyMap;
use super::style::AppStyle;
use super::{Exit, KadeuApp};

pub struct DeckBrowser {
    root: PathBuf,
    relative_path: PathBuf,
    collection: FileCollection,
    index: usize,
}

impl TryFrom<PathBuf> for DeckBrowser {
    type Error = std::io::Error;
    fn try_from(root: PathBuf) -> Result<Self, Self::Error> {
        let collection = FileCollection::try_from(root.clone())?;
        let browser = Self {
            relative_path: PathBuf::from("/"),
            root,
            collection,
            index: 0,
        };

        Ok(browser)
    }
}
// this works?
#[derive(Default, Debug, Clone)]
struct FileCollection {
    root: PathBuf,
    subpaths: Vec<PathBuf>,
    index: usize,
}

impl TryFrom<PathBuf> for FileCollection {
    type Error = std::io::Error;
    fn try_from(root: PathBuf) -> Result<Self, Self::Error> {
        let mut subpaths: Vec<PathBuf> = vec![];

        for entry in fs::read_dir(&root)? {
            let entry = entry?;
            if entry.path().file_name().is_some() {
                subpaths.push(entry.path());
            }
        }

        Ok(Self {
            root,
            subpaths,
            index: 0,
        })
    }
}

impl FileCollection {
    fn inc(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        }
    }

    fn root(&self) -> &PathBuf {
        &self.root
    }

    pub fn peek_index(&self) -> &PathBuf {
        self.subpaths.get(self.index).unwrap()
    }

    pub fn index_filename(&self) -> OsString {
        self.peek_index().file_name().unwrap().to_os_string()
    }

    fn dec(&mut self) {
        if self.index < self.subpaths.len() - 1 {
            self.index += 1;
        }
    }

    pub fn traverse(&mut self) -> std::io::Result<()> {
        let path = self.peek_index().clone();
        if path.is_dir() {
            let mut collection = FileCollection::try_from(path)?;
            swap(self, &mut collection);
            self.index = 0;
        }
        Ok(())
    }

    pub fn reverse(&mut self) -> std::io::Result<()> {
        if let Some(parent) = self.root().parent() {
            let path = parent.to_path_buf();
            let mut collection = FileCollection::try_from(path)?;
            swap(self, &mut collection);
        }

        Ok(())
    }

    fn view(&self) -> Vec<String> {
        // all subpaths are filtered to be some at this point.
        self.subpaths
            .iter()
            .map(|path| path.file_name().unwrap().to_string_lossy().to_string())
            .collect()
    }
}

impl KadeuApp for DeckBrowser {
    fn handle_input(&mut self, input: Option<&Input>) -> std::io::Result<Exit> {
        let Some(input) = input else {
            return Ok(Exit::None);
        };
        let exit = match input {
            Input::Up => {
                self.collection.inc();
                Exit::None
            }

            Input::Down => {
                self.collection.dec();
                Exit::None
            }

            Input::Backspace => {
                if self.collection.root() != &self.root {
                    self.relative_path.pop();
                    self.collection.reverse()?;
                }
                Exit::None
            }
            Input::Select => {
                if self.collection.peek_index().is_dir() {
                    self.relative_path.push(self.collection.index_filename());
                    self.collection.traverse()?;
                    Exit::None
                } else {
                    Exit::Drop
                }
            }
            _ => Exit::None,
        };
        let action = exit;
        Ok(action)
    }
    fn render<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
        style: &AppStyle,
    ) -> std::io::Result<()> {
        let view = self.collection.view();
        let items = view.into_iter().map(|item| Text::from(item));
        let title = self.relative_path.as_os_str().to_string_lossy().to_string();
        let list = style
            .list(items)
            .highlight_symbol("> ")
            .block(style.block().title(title));
        let mut state = ListState::default();
        state.select(Some(self.collection.index));
        terminal.draw(|frame| {
            frame.render_stateful_widget(list, frame.area(), &mut state);
        })?;
        Ok(())
    }
    fn style(&self) -> AppStyle {
        AppStyle::default().bg(Color::White)
    }

    fn keymap(&self) -> KeyMap {
        let mut map = KeyMap::new();
        map.insert(KeyCode::Char('q'), Input::Quit);
        map.insert(KeyCode::Char('j'), Input::Down);
        map.insert(KeyCode::Char('k'), Input::Up);
        map.insert(KeyCode::Enter, Input::Select);
        map.insert(KeyCode::Backspace, Input::Backspace);
        map
    }

    fn drop(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl DeckBrowser {
    pub fn current_path_is_file(&self) -> bool {
        self.collection.peek_index().is_file()
    }

    pub fn current_path(&self) -> PathBuf {
        self.collection.peek_index().clone()
    }
    pub fn is_deck<T: Kadeu + DeserializeOwned>(&self) -> bool {
        let file = self.collection.peek_index();

        if file.is_dir() {
            return false;
        }

        let trials = [FileType::Json(file.clone())];

        for trial in trials {
            if trial.load::<Deck<T>>().is_ok() {
                return true;
            }
        }

        false
    }
}
