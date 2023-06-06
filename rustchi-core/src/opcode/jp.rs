use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum JP {
    S(u8),
    C(u8),
    NC(u8),
    Z(u8),
    NZ(u8),
}

impl fmt::Display for JP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JP::S(s) => write!(f, "JP {}", s),
            JP::C(s) => write!(f, "JP C {}", s),
            JP::NC(s) => write!(f, "JP NC {}", s),
            JP::Z(s) => write!(f, "JP Z {}", s),
            JP::NZ(s) => write!(f, "JP NZ {}", s),
        }
    }
}
