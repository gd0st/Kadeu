use std::fs;
use std::path::PathBuf;

struct DeckBrowser {
    root: String,
}

impl From<PathBuf> for DeckBrowser {
    fn from(value: PathBuf) -> Self {
        let root = value.display().to_string();
        Self { root }
    }
}

impl DeckBrowser {
    pub fn view(&self) -> std::io::Result<Vec<String>> {
        let mut view = vec![];
        for entry in fs::read_dir(PathBuf::from(&self.root))? {
            let path = entry?.path();
            if path.starts_with(".") {
                continue;
            }

            let mut name = path.file_name().unwrap().to_string_lossy().to_string();
            if path.is_dir() {
                name.push('/');
            }
            view.push(name)
        }
        Ok(view)
    }
}
