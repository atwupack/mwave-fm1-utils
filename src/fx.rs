use wmidi::{ControlFunction, U7};

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Effect(pub ControlFunction);

impl Effect {
    pub const FILTER: Self = Self(ControlFunction(U7::from_u8_lossy(0)));
    pub const REVERB: Self = Self(ControlFunction(U7::from_u8_lossy(4)));
    pub const DELAY: Self = Self(ControlFunction(U7::from_u8_lossy(8)));
    pub const DISTORTION: Self = Self(ControlFunction(U7::from_u8_lossy(12)));
    pub const CHORUS: Self = Self(ControlFunction(U7::from_u8_lossy(16)));
    pub const PHASER: Self = Self(ControlFunction(U7::from_u8_lossy(32)));
}