use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum JP {
    S(u8),
    C(u8),
    NC(u8),
    Z(u8),
    NZ(u8),
    BA,
}

impl fmt::Display for JP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JP::S(s) => write!(f, "JP {:#04X}", s),
            JP::C(s) => write!(f, "JP C {:#04X}", s),
            JP::NC(s) => write!(f, "JP NC {:#04X}", s),
            JP::Z(s) => write!(f, "JP Z {:#04X}", s),
            JP::NZ(s) => write!(f, "JP NZ {:#04X}", s),
            JP::BA => write!(f, "JP BA"),
        }
    }
}
