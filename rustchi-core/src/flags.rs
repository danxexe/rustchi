use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Copy)]
    pub struct Flags: u8 {
        const C = 0x1 << 0;
        const Z = 0x1 << 1;
        const D = 0x1 << 2;
        const I = 0x1 << 3;
    }
}
