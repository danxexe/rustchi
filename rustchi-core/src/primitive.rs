#![allow(non_camel_case_types)]

#[derive(Clone, Copy)]
pub struct u1(u8);
impl From<u8> for u1 {
    fn from(item: u8) -> Self {
        Self(item)
    }
}

#[derive(Clone, Copy)]
pub struct u4(u8);
impl From<u8> for u4 {
    fn from(item: u8) -> Self {
        Self(item)
    }
}
impl From<u4> for u8 {
    fn from(item: u4) -> Self {
        item.0
    }
}

#[derive(Clone, Copy)]
pub struct u12(u16);
impl From<u16> for u12 {
    fn from(item: u16) -> Self {
        Self(item)
    }
}

pub trait GetNibble {
    fn high(&self) -> u4;
    fn low(&self) -> u4;
    fn with_high(&self, data: u4) -> Self;
    fn with_low(&self, data: u4) -> Self;
}
impl GetNibble for u8 {
    fn high(&self) -> u4 {
        (self & 0xF0 >> 4).into()
    }
    fn low(&self) -> u4 {
        (self & 0x0F).into()
    }
    fn with_high(&self, data: u4) -> u8 {
        let high = u8::from(data) << 4;
        let low = u8::from(self.low());
        high | low
    }
    fn with_low(&self, data: u4) -> Self {
        let high = u8::from(self.high()) << 4;
        let low = u8::from(data);
        high | low
    }
}
