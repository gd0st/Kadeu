use core::fmt;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json;
use std::{
    ffi::{self, OsStr},
    fs::{self, File, OpenOptions},
    path::{Path, PathBuf},
};

use crate::{app::Deck, cli::Config};

#[derive(Debug, Clone)]
pub enum ImportEntry {
    File(PathBuf),
    Collection(PathBuf),
}

impl ImportEntry {
    fn filename<T>(&self) -> Option<&ffi::OsStr> {
        match self {
            Self::File(path) => path.file_name(),
            Self::Collection(path) => {
                if let Some(filename) = path.file_name() {
                    Some(filename)
                } else {
                    None
                }
            }
        }
    }
}

pub enum FileType {
    Json(PathBuf),
}

pub struct Directories {
    imports: String,
}

pub fn list_directory(path: &PathBuf) -> std::io::Result<Vec<ImportEntry>> {
    let mut entries = vec![];

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            entries.push(ImportEntry::Collection(path));
        } else if path.is_file() {
            entries.push(ImportEntry::File(path));
        }
    }
    Ok(entries)
}

pub fn convert_to_path<T: DeserializeOwned + Serialize>(
    source: FileType,
    destination: FileType,
) -> std::io::Result<()> {
    let item: T = source.load()?;
    destination.save(&item)
}

impl Default for Directories {
    fn default() -> Self {
        Self {
            imports: "./imported".to_string(),
        }
    }
}

impl FileType {
    pub fn json(path: &PathBuf) -> Self {
        Self::Json(path.clone())
    }
    pub fn load<T: DeserializeOwned>(self) -> std::io::Result<T> {
        match self {
            Self::Json(path) => {
                let reader = OpenOptions::new().read(true).open(path)?;
                let item: T = serde_json::from_reader(reader)?;
                Ok(item)
            }
        }
    }

    pub fn save<T: Serialize>(self, item: &T) -> std::io::Result<()> {
        match self {
            Self::Json(path) => {
                let writer = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open(path)?;
                let _ = serde_json::to_writer(writer, item)?;
                Ok(())
            }
        }
    }
}

pub fn list_files<T>(path: &PathBuf) -> Vec<T> {
    todo!("List the files here");
    vec![]
}
