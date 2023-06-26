use crate::state::State;

use std::fmt::Display;

pub trait Op: Exec + Cycles + Display {}

pub trait Exec {
    fn exec(&self, state: &mut State);
    fn interruptible(&self) -> bool {
        true
    }
}

pub trait Cycles {
    fn cycles(&self) -> u32;
}
