use crate::primitive::u4;

#[derive(Debug, Clone, Copy)]
pub enum IdentU1 {
    PCB,
    NBP,
}

#[derive(Debug, Clone, Copy)]
pub enum IdentU4 {
    A,
    B,
    MX,
    MY,
    Mn(u4),
    MSP,
    XP,
    XH,
    XL,
    YP,
    YH,
    YL,
    F,
    PCP,
    NPP,
    Imm(u4), // Should immediate really be part of IdentU4? Maybe we need to split into Dest and Source operands.
}

#[derive(Debug, Clone, Copy)]
pub enum IdentU8 {
    PCS,
}

#[derive(Debug, Clone, Copy)]
pub enum IdentU12 {
    X,
    Y,
}
