#![macro_use]

macro_rules! u4 {
    ($from:expr) => {{
        let value: crate::primitive::u4 = $from.try_into().unwrap();
        value
    }}
}

macro_rules! u12 {
    ($from:expr) => {{
        let value: crate::primitive::u12 = $from.try_into().unwrap();
        value
    }}
}

macro_rules! rq {
    ($from:expr) => {{
        let rq: crate::opcode::rq::RQ = RQ::from(u4![$from]);
        rq
    }}
}
