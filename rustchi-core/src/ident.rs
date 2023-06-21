use crate::primitive::u4;

pub enum Ident {
    U4(IdentU4),
    U8(IdentU8),
}

impl From<IdentU4> for Ident {
    fn from(value: IdentU4) -> Self {
        Ident::U4(value)
    }
}

impl From<IdentU8> for Ident {
    fn from(value: IdentU8) -> Self {
        Ident::U8(value)
    }
}

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
    XHL,
    YHL,
    Imm(u8),
}

#[derive(Debug, Clone, Copy)]
pub enum IdentU12 {
    X,
    Y,
}

pub trait FetchIdent<I,T> {
    fn fetch(&self, ident: I) -> T;
}

pub trait SetIdent<I,T> {
    fn set(&mut self, ident: I, value: T) -> &mut Self;
}
