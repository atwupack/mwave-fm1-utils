use std::hash::Hash;
use wmidi::{ControlFunction, ControlValue, U7};

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Effect(pub(crate) ControlFunction);

impl Effect {
    pub const FILTER: Self = Self(ControlFunction(U7::from_u8_lossy(0)));
    pub const REVERB: Self = Self(ControlFunction(U7::from_u8_lossy(4)));
    pub const DELAY: Self = Self(ControlFunction(U7::from_u8_lossy(8)));
    pub const DISTORTION: Self = Self(ControlFunction(U7::from_u8_lossy(12)));
    pub const CHORUS: Self = Self(ControlFunction(U7::from_u8_lossy(16)));
    pub const PHASER: Self = Self(ControlFunction(U7::from_u8_lossy(20)));
}

pub trait EffectParameter {
    fn control_function(&self) -> ControlFunction;
    fn control_value(&self) -> ControlValue;
}

#[repr(u8)]
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReverbType {
    Room = 0,
    Hall = 1,
    Plate = 2,
}

impl EffectParameter for ReverbType {
    fn control_function(&self) -> ControlFunction {
        ControlFunction(U7::from_u8_lossy(5))
    }

    fn control_value(&self) -> ControlValue {
        U7::from_u8_lossy(*self as u8)
    }
}

pub struct ReverbDecay(u8);

impl EffectParameter for ReverbDecay {
    fn control_function(&self) -> ControlFunction {
        ControlFunction(U7::from_u8_lossy(6))
    }

    fn control_value(&self) -> ControlValue {
        U7::from_u8_lossy(self.0.min(100))
    }
}


#[repr(u8)]
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum FilterType {
    LowPass = 0,
    BandPass = 1,
    HighPass = 2,
}

impl EffectParameter for FilterType {
    fn control_function(&self) -> ControlFunction {
        ControlFunction(U7::from_u8_lossy(1))
    }

    fn control_value(&self) -> ControlValue {
        U7::from_u8_lossy(*self as u8)
    }
}
