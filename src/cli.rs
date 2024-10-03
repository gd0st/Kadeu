use std::ffi::OsStr;
use std::{default, env, fs::OpenOptions, io::Read, path::PathBuf};
use toml;

use clap::{self, Arg, Parser};
use serde::{Deserialize, Serialize};

#[cfg(target_os = "linux")]
const HOME_VAR: &str = "HOME";
#[cfg(target_os = "linux")]
const SYSTEM_CONFIG: &str = "/etc";

mod fs {
    // Anything file system related would go here?
    // Maybe just mod fs would be better here imo.
    // [ ] TODO just add mod fs
}

#[derive(Debug, Clone, PartialEq)]
enum Error<'a> {
    NoHome,
    Unkown(&'a str),
}

impl<'a> Into<std::io::Error> for Error<'a> {
    fn into(self) -> std::io::Error {
        match self {
            Self::NoHome => std::io::Error::other("the home directory was unable to be sourced"),
            Self::Unkown(err) => std::io::Error::other(err),
        }
    }
}

impl<'a> Default for Error<'a> {
    fn default() -> Self {
        Self::Unkown("an unkown error has occured :(")
    }
}

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    #[arg(long)]
    pub debug: bool,
    #[arg(long, short)]
    config: Option<PathBuf>,
    #[command(subcommand)]
    pub subcommand: Subcommand,
}

impl Args {
    pub fn read_config(&self) -> std::io::Result<Config> {
        let path = if let Some(path) = &self.config {
            path.clone()
        } else {
            // ?? not meant to be run like this...
            let home_path = env::var(HOME_VAR).unwrap_or(SYSTEM_CONFIG.to_string());
            format!("{}/.config/kadeu/kadeu.toml", home_path).into()
        };

        let mut buf = String::new();
        let _ = OpenOptions::new()
            .read(true)
            .open(path)?
            .read_to_string(&mut buf)?;

        match toml::from_str::<Config>(&buf) {
            Ok(config) => Ok(config),
            Err(e) => {
                let message = e.message();
                let format = format!("Error parsing configuration file: {}", message);
                Err(std::io::Error::other(format))
            }
        }
    }
}

#[derive(clap::Subcommand, Debug, Clone, Default)]
pub enum Subcommand {
    Show,
    Import {
        path: PathBuf,
    },
    Config,
    #[default]
    Browse,
    Source {
        path: PathBuf,
    },
    Run {
        name: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    import_directory: Option<PathBuf>,
    default_strategy: String,
    backend: Option<String>,
}

impl Config {
    pub fn import_directory(&self) -> PathBuf {
        if let Some(path) = &self.import_directory {
            path.clone()
        } else {
            if let Ok(home_path) = env::var(HOME_VAR) {
                let mut path = PathBuf::from(home_path);
                path.push(".config/kadeu/imports");
                path
            } else {
                let mut path = PathBuf::from(SYSTEM_CONFIG);
                path.push("kadeu/imports");
                path
            }
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            import_directory: None,
            default_strategy: "Random".to_string(),
            backend: Some("crossterm".to_string()),
        }
    }
}
