#![macro_use]

macro_rules! u4 {
    ($from:expr) => {{
        let value: crate::primitive::u4 = $from.try_into().unwrap();
        value
    }}
}

macro_rules! u8 {
    ($from:expr) => {{
        let value: u8 = $from.try_into().unwrap();
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

macro_rules! op {
    ($opcode:expr) => {{
        Opcode::Op(Rc::new($opcode))
    }}
}

macro_rules! def_opcode {
    ($v:vis $keyword:ident $name:ident $($tt:tt)*) => {
        $v $keyword $name $($tt)*

        type T = $name;
        const NAME: &str = stringify!($name);

        use crate::opcode::Op;
        impl Op for T {}
    };
}
