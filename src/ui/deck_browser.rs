use crate::ui::inputs::Input;
use crossterm::event::KeyCode;
use ratatui::prelude::Backend;
use ratatui::style::Stylize;
use ratatui::text::Text;
use ratatui::widgets::{Block, List, ListState};
use ratatui::Terminal;
use std::ffi::OsString;
use std::fs;
use std::mem::swap;
use std::path::PathBuf;

use super::inputs::KeyMap;
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
            relative_path: PathBuf::new(),
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

        // TODO Present a view of the subpath
        // TODO Allow the cursor to move up and down.
        // TODO Filter out hidden paths.
        // TODO move into sub paths
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

    pub fn peek(&self) -> &PathBuf {
        self.subpaths.get(self.index).unwrap()
    }

    pub fn index_filename(&self) -> OsString {
        self.peek().file_name().unwrap().to_os_string()
    }

    fn dec(&mut self) {
        if self.index < self.subpaths.len() - 1 {
            self.index += 1;
        }
    }

    pub fn traverse(&mut self) -> std::io::Result<()> {
        let path = self.peek().clone();

        if path.is_dir() {
            let mut collection = FileCollection::try_from(path)?;
            // TODO for some reason traversing breaks everything..
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

    fn subpaths(&self) -> Vec<(String, &PathBuf)> {
        self.subpaths
            .iter()
            .map(|path| {
                let filename = path.file_name().unwrap().to_string_lossy().to_string();
                (filename, path)
            })
            .collect()
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
                // Somehow completley breaks rendering after this....
                if self.collection.peek().is_dir() {
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
    fn render<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> std::io::Result<()> {
        let view = self.collection.view();
        let items = view.into_iter().map(|item| Text::from(item));
        let title = self.relative_path.as_os_str().to_string_lossy().to_string();
        let block = Block::default().title(title);
        let list = List::new(items)
            .on_black()
            .white()
            .highlight_symbol("> ")
            .block(block);
        let mut state = ListState::default();
        state.select(Some(self.collection.index));
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
        map.insert(KeyCode::Backspace, Input::Backspace);
        map
    }

    fn drop(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl DeckBrowser {
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
    pub fn view(&self) -> Vec<String> {
        self.collection
            .subpaths()
            .iter()
            .map(|(filename, path)| {
                if path.is_dir() {
                    format!("{} >", filename)
                } else {
                    format!("{}", filename)
                }
            })
            .collect()
    }

    /// Will change it's state to browse the child folder
    /// if the self.index is discovered to be on a path
    /// otherwise nothing happens
    pub fn traverse(&mut self) -> std::io::Result<()> {
        // TODO Fix this to not return an Action...
        // Actions will be done by handle_input instead. Bad code right now
        // still kinda a messy function

        if self.collection.peek().is_dir() {}

        Ok(())
    }
}
