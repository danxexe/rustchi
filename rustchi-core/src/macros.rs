#![macro_use]

macro_rules! u4 {
    ($from:expr) => {{
        let from: u16 = $from;
        let value: u4 = from.try_into().unwrap();
        value
    }}
}
