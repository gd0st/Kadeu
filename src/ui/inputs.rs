use std::{collections::HashMap, default};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use super::Debugger;

#[derive(Debug, Clone, PartialEq)]
pub enum Input {
    Up,
    Down,
    Continue,
    Select,
    Escape,
    Quit,
}

pub type KeyMap = HashMap<KeyCode, Input>;

fn get_input<'a>(map: &'a KeyMap, code: &KeyCode) -> Option<&'a Input> {
    map.get(code)
}

#[derive(Default)]
pub struct Events {
    keymap: KeyMap,
}

impl From<KeyMap> for Events {
    fn from(keymap: KeyMap) -> Self {
        Self { keymap }
    }
}

type Timeout = std::time::Duration;

impl Events {
    pub fn poll(&self, timeout: u64) -> std::io::Result<Option<&Input>> {
        // listens for an event and distrbutes it to its listenrs.

        //shoot looks ugly af
        let event = if event::poll(Timeout::from_millis(timeout))? {
            if let Event::Key(event) = event::read()? {
                match event.kind {
                    event::KeyEventKind::Press => get_input(&self.keymap, &event.code),
                    _ => None,
                }
            } else {
                None
            }
        } else {
            None
        };

        Ok(event)
    }
}

pub trait EventListener {
    fn on_event(&mut self, input: &Input);
}

struct Eventer {}

mod tests {

    struct Foobar;
    use super::Input;

    trait EventListener {
        fn add_event_listener<'a>(&mut self, event: super::Input, result: fn(&Self, Input)) {
            result(self, event)
        }
    }

    impl EventListener for Foobar {}

    // very cool but useless
    #[test]
    fn add_event_listener() {
        let mut foobar = Foobar {};
        foobar.add_event_listener(Input::Up, |this, event| assert!(true));
    }
}
