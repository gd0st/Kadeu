use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

enum Input {
    Up,
    Down,
    Continue,
    Select,
}

type KeyMap = HashMap<KeyCode, Input>;
