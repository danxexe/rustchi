use crate::prelude::*;
use std::fmt;

def_opcode! {
    #[derive(Debug, Clone, Copy)]
    pub enum ACP {
        X(RQ),
        Y(RQ),
    }
}

impl fmt::Display for T {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::X(r) => write!(f, "{NAME}X MX {}", r),
            Self::Y(r) => write!(f, "{NAME}Y MY {}", r),
        }
    }
}

impl T {
    pub fn source(&self) -> (IdentU12, IdentU4, IdentU4) {
        match *self {
            Self::X(r) => (IdentU12::X, IdentU4::MX, r.into()),
            Self::Y(r) => (IdentU12::Y, IdentU4::MY, r.into()),
        }
    }
}

impl Exec for T {
    fn exec(&self, state: &mut State) {
        let (ii, ia, ib) = self.source();
        let i: u12 = state.fetch(ii);
        let a: u8 = state.fetch(ia).into();
        let b: u8 = state.fetch(ib).into();
        let c: u8 = state.fetch(Flags::C);
        let d: bool = state.fetch(Flags::D) != 0;
        let val = a + b + c;

        let (new_val, carry) = if d {
            if val >= 10 {
                (u4![(val - 10) & 0xF], true)
            } else {
                (u4![val], false)
            } 
        } else {
            (u4![val & 0xF], val > 0xF)
        };

        state
            .set(ia, new_val)
            .set(ii, u12![i + u12![1]])
            .set_flag(Flags::C, carry)
            .set_flag(Flags::Z, new_val == u4![0]);
    }
}

impl Cycles for T {
    fn cycles(&self) -> u32 { 7 }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::state::State;

    #[test]
    fn acp() {
        let cases = vec![
            ((0, 0, 0), (0, 0)),
            ((1, 1, 0), (2, 0)),
            ((1, 1, 1), (3, 0)),
            ((0xE, 1, 0), (0xF, 0)),
            ((0xE, 0, 1), (0xF, 0)),
            ((0xE, 1, 1), (0, 1)),
            ((0xF, 1, 1), (1, 1)),
        ];

        for ((mx, a, c), (new_mx, new_c)) in cases {
            let mut state = State::new();

            let x_before = state.fetch(IdentU12::X);
            state.set(IdentU4::MX, u4![mx]);
            state.set(IdentU4::A, u4![a]);
            state.set_flag(Flags::C, c == 1);
    
            let op = ACP::X(RQ::A);
            op.exec(&mut state);
    
            assert_eq!(state.fetch(IdentU12::X), x_before + u12![1]);
            state.set(IdentU12::X, x_before);
            assert_eq!(state.fetch(IdentU4::MX), u4![new_mx]);
            assert_eq!(state.fetch(Flags::C), new_c);
        }
    }
}
