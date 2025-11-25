use crate::graphics;
use std::time::Instant;

enum SelectMode {
    Dpad,
    Buttons,
}

pub enum Button {
    Up,
    Down,
    Left,
    Right,
    Start,
    Select,
    A,
    B,
}

pub enum ButtonState {
    Up,
    Down,
}

pub struct Hardware {
    joypad: Joypad,
}
impl Hardware {
    pub fn new() -> Self {
        Self {
            joypad: Joypad::new(),
        }
    }
}

impl Default for Hardware {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Joypad {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    start: bool,
    select: bool,
    a: bool,
    b: bool,
}

impl Joypad {
    pub fn new() -> Self {
        Self {
            up: false,
            down: false,
            left: false,
            right: false,
            start: false,
            select: false,
            a: false,
            b: false,
        }
    }
    pub fn set(&mut self, button: Button, state: ButtonState) {
        let button = &mut match button {
            Button::Up => self.up,
            Button::Down => self.down,
            Button::Left => self.left,
            Button::Right => self.right,
            Button::Start => self.start,
            Button::Select => self.select,
            Button::A => self.a,
            Button::B => self.b,
        };
        *button = match state {
            ButtonState::Up => false,
            ButtonState::Down => true,
        };
    }
}
