use crate::prelude::*;
use std::fmt;

def_opcode! {
    #[derive(Debug, Clone, Copy)]
    pub enum INC {
        X,
        Y,
    }
}

impl fmt::Display for T {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::X => write!(f, "{NAME} X"),
            Self::Y => write!(f, "{NAME} Y"),
        }
    }
}

impl From<T> for IdentU12 {
    fn from(value: T) -> IdentU12 {
        match value {
            INC::X => Self::X,
            INC::Y => Self::Y,
        }
    }
}

impl Exec for T {
    fn exec(&self, state: &mut State) {
        let ident = IdentU12::from(*self);
        state.set_u12(ident, state.fetch_u12(ident) + u12![1]);
    }
}

impl Cycles for T {
    fn cycles(&self) -> u32 { 5 }
}
