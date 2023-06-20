use crate::{
    opcode::*
};

def_opcode! {
    pub enum LD {
        r_XP(RQ),
        r_XH(RQ),
        r_XL(RQ),
        r_YP(RQ),
        r_YH(RQ),
        r_YL(RQ),
        r_i(RQ, u4),
        r_q(RQ, RQ),
        A_Mn(u4),
        B_Mn(u4),
        Mn_A(u4),
        Mn_B(u4),
    }
}

impl fmt::Display for T {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::r_XP(r) => write!(f, "{NAME} {} XP", r),
            Self::r_XH(r) => write!(f, "{NAME} {} XH", r),
            Self::r_XL(r) => write!(f, "{NAME} {} XL", r),
            Self::r_YP(r) => write!(f, "{NAME} {} YP", r),
            Self::r_YH(r) => write!(f, "{NAME} {} YH", r),
            Self::r_YL(r) => write!(f, "{NAME} {} YL", r),
            Self::r_i(r, i) => write!(f, "{NAME} {} {:#03X}", r, i),
            Self::r_q(r, q) => write!(f, "{NAME} {} {}", r, q),
            Self::A_Mn(n) => write!(f, "{NAME} A M{}", n),
            Self::B_Mn(n) => write!(f, "{NAME} B M{}", n),
            Self::Mn_A(n) => write!(f, "{NAME} M{} A", n),
            Self::Mn_B(n) => write!(f, "{NAME} M{} B", n),
        }
    }
}

impl LD {
    pub fn dest(&self) -> IdentU4 {
        match self {
            Self::r_XP(r) => IdentU4::from(*r),
            Self::r_XH(r) => IdentU4::from(*r),
            Self::r_XL(r) => IdentU4::from(*r),
            Self::r_YP(r) => IdentU4::from(*r),
            Self::r_YH(r) => IdentU4::from(*r),
            Self::r_YL(r) => IdentU4::from(*r),
            Self::r_i(r, _i) => IdentU4::from(*r),
            Self::r_q(r, _q) => IdentU4::from(*r),
            Self::A_Mn(_n) => IdentU4::A,
            Self::B_Mn(_n) => IdentU4::B,
            Self::Mn_A(n) => IdentU4::Mn(*n),
            Self::Mn_B(n) => IdentU4::Mn(*n),
        }
    }

    pub fn source(&self) -> IdentU4 {
        match self {
            Self::r_XP(_) => IdentU4::XP,
            Self::r_XH(_) => IdentU4::XH,
            Self::r_XL(_) => IdentU4::XL,
            Self::r_YP(_) => IdentU4::YP,
            Self::r_YH(_) => IdentU4::YH,
            Self::r_YL(_) => IdentU4::YL,
            Self::r_i(_r, i) => IdentU4::Imm(*i),
            Self::r_q(_r, q) => IdentU4::from(*q),
            Self::A_Mn(n) => IdentU4::Mn(*n),
            Self::B_Mn(n) => IdentU4::Mn(*n),
            Self::Mn_A(_n) => IdentU4::A,
            Self::Mn_B(_n) => IdentU4::B,
        }
    }
}

impl Exec for T {
    fn exec(&self, state: &mut crate::state::State) {
        // if let IdentU4::Mn(n) = self.source() {
        //     todo!()
        // }

        state.set_u4(self.dest(), state.fetch_u4(self.source()));
    }
}

impl Cycles for T {
    fn cycles(&self) -> u32 { 5 }
}
