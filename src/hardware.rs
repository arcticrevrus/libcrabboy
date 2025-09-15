use std::sync::{Arc, Mutex};

#[derive(Debug)]
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

#[derive(Debug)]
pub enum ButtonState {
    Up,
    Down,
}

pub struct Hardware {
    pub joypad: Arc<Mutex<Joypad>>,
}
impl Hardware {
    pub fn new() -> Self {
        Self {
            joypad: Arc::new(Mutex::new(Joypad::new())),
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
        let target: &mut bool = match button {
            Button::Up => &mut self.up,
            Button::Down => &mut self.down,
            Button::Left => &mut self.left,
            Button::Right => &mut self.right,
            Button::Start => &mut self.start,
            Button::Select => &mut self.select,
            Button::A => &mut self.a,
            Button::B => &mut self.b,
        };
        *target = matches!(state, ButtonState::Down);
        println!("{button:?}: {state:?}")
    }

    pub fn get(&self, button: Button) -> ButtonState {
        let pressed = match button {
            Button::Up => self.up,
            Button::Down => self.down,
            Button::Left => self.left,
            Button::Right => self.right,
            Button::Start => self.start,
            Button::Select => self.select,
            Button::A => self.a,
            Button::B => self.b,
        };
        if pressed {
            ButtonState::Down
        } else {
            ButtonState::Up
        }
    }
}

impl Default for Joypad {
    fn default() -> Self {
        Self::new()
    }
}
