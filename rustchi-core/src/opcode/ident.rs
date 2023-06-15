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
