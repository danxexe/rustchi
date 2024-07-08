use crate::prelude::*;

#[derive(Clone)]
pub struct Input {
    pub state: u4,
}

impl Input {
    pub fn all_high() -> Self {
        Self {state: u4![0b1111]}
    }

    pub fn with_button_pressed(&self, button: Button) -> Self {
        Self {state: !(!self.state | button.to_u4())}
    }

    pub fn with_button_released(&self, button: Button) -> Self {
        Self {state: (self.state | button.to_u4())}
    }

    pub fn is_button_pressed(&self, button: Button) -> bool {
        (!self.state & button.to_u4()) != u4![0]
    }
}

pub enum Button {
    A,
    B,
    C,
}

impl Button {
    pub fn to_u4(&self) -> u4 {
        match self {
            Button::A => u4![0b0100],
            Button::B => u4![0b0010],
            Button::C => u4![0b0001],
        }
    }
}