use std::fmt;

use super::IdentU12;

#[derive(Debug, Clone, Copy)]
pub enum INC {
    X,
    Y,
}

impl fmt::Display for INC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::X => write!(f, "INC X"),
            Self::Y => write!(f, "INC Y"),
        }
    }
}

impl From<INC> for IdentU12 {
    fn from(value: INC) -> IdentU12 {
        match value {
            INC::X => Self::X,
            INC::Y => Self::Y,
        }
    }
}
