use std::hash::Hash;
use wmidi::{ControlFunction, ControlValue, U7};

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
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
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
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

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct FilterCutoff(u8);

impl EffectParameter for FilterCutoff {
    fn control_function(&self) -> ControlFunction {
        ControlFunction(U7::from_u8_lossy(2))
    }

    fn control_value(&self) -> ControlValue {
        U7::from_u8_lossy(self.0.min(107))
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct FilterQ(u8);

impl EffectParameter for FilterQ {
    fn control_function(&self) -> ControlFunction {
        ControlFunction(U7::from_u8_lossy(3))
    }

    fn control_value(&self) -> ControlValue {
        U7::from_u8_lossy(self.0.min(10))
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
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

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct ReverbDecay(u8);

impl EffectParameter for ReverbDecay {
    fn control_function(&self) -> ControlFunction {
        ControlFunction(U7::from_u8_lossy(6))
    }

    fn control_value(&self) -> ControlValue {
        U7::from_u8_lossy(self.0.min(100))
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct ReverbMix(u8);

impl EffectParameter for ReverbMix {
    fn control_function(&self) -> ControlFunction {
        ControlFunction(U7::from_u8_lossy(7))
    }

    fn control_value(&self) -> ControlValue {
        U7::from_u8_lossy(self.0.min(100))
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct DelayDecay(u8);

impl EffectParameter for DelayDecay {
    fn control_function(&self) -> ControlFunction {
        ControlFunction(U7::from_u8_lossy(9))
    }

    fn control_value(&self) -> ControlValue {
        U7::from_u8_lossy(self.0.min(100))
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct DelayRate(u8);

impl EffectParameter for DelayRate {
    fn control_function(&self) -> ControlFunction {
        ControlFunction(U7::from_u8_lossy(10))
    }

    fn control_value(&self) -> ControlValue {
        U7::from_u8_lossy(self.0.min(100))
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct DelayMix(u8);

impl EffectParameter for DelayMix {
    fn control_function(&self) -> ControlFunction {
        ControlFunction(U7::from_u8_lossy(11))
    }

    fn control_value(&self) -> ControlValue {
        U7::from_u8_lossy(self.0.min(100))
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct DistortionGain(u8);

impl EffectParameter for DistortionGain {
    fn control_function(&self) -> ControlFunction {
        ControlFunction(U7::from_u8_lossy(13))
    }

    fn control_value(&self) -> ControlValue {
        U7::from_u8_lossy(self.0.min(100))
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct DistortionTone(u8);

impl EffectParameter for DistortionTone {
    fn control_function(&self) -> ControlFunction {
        ControlFunction(U7::from_u8_lossy(14))
    }

    fn control_value(&self) -> ControlValue {
        U7::from_u8_lossy(self.0.min(100))
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct DistortionLevel(u8);

impl EffectParameter for DistortionLevel {
    fn control_function(&self) -> ControlFunction {
        ControlFunction(U7::from_u8_lossy(15))
    }

    fn control_value(&self) -> ControlValue {
        U7::from_u8_lossy(self.0.min(100))
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct ChorusFreq(u8);

impl EffectParameter for ChorusFreq {
    fn control_function(&self) -> ControlFunction {
        ControlFunction(U7::from_u8_lossy(17))
    }

    fn control_value(&self) -> ControlValue {
        U7::from_u8_lossy(self.0.min(100))
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct ChorusDepth(u8);

impl EffectParameter for ChorusDepth {
    fn control_function(&self) -> ControlFunction {
        ControlFunction(U7::from_u8_lossy(18))
    }

    fn control_value(&self) -> ControlValue {
        U7::from_u8_lossy(self.0.min(100))
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct ChorusMix(u8);

impl EffectParameter for ChorusMix {
    fn control_function(&self) -> ControlFunction {
        ControlFunction(U7::from_u8_lossy(19))
    }

    fn control_value(&self) -> ControlValue {
        U7::from_u8_lossy(self.0.min(100))
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct PhaserFreq(u8);

impl EffectParameter for PhaserFreq {
    fn control_function(&self) -> ControlFunction {
        ControlFunction(U7::from_u8_lossy(21))
    }

    fn control_value(&self) -> ControlValue {
        U7::from_u8_lossy(self.0.min(100))
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct PhaserDepth(u8);

impl EffectParameter for PhaserDepth {
    fn control_function(&self) -> ControlFunction {
        ControlFunction(U7::from_u8_lossy(22))
    }

    fn control_value(&self) -> ControlValue {
        U7::from_u8_lossy(self.0.min(100))
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct PhaserMix(u8);

impl EffectParameter for PhaserMix {
    fn control_function(&self) -> ControlFunction {
        ControlFunction(U7::from_u8_lossy(23))
    }

    fn control_value(&self) -> ControlValue {
        U7::from_u8_lossy(self.0.min(100))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_parameters() {
        assert_eq!(FilterType::LowPass.control_function(), ControlFunction(U7::from_u8_lossy(1)));
        assert_eq!(FilterType::LowPass.control_value(), U7::from_u8_lossy(0));
        assert_eq!(FilterType::BandPass.control_function(), ControlFunction(U7::from_u8_lossy(1)));
        assert_eq!(FilterType::BandPass.control_value(), U7::from_u8_lossy(1));
        assert_eq!(FilterType::HighPass.control_function(), ControlFunction(U7::from_u8_lossy(1)));
        assert_eq!(FilterType::HighPass.control_value(), U7::from_u8_lossy(2));

        let cutoff = FilterCutoff(50);
        assert_eq!(cutoff.control_function(), ControlFunction(U7::from_u8_lossy(2)));
        assert_eq!(cutoff.control_value(), U7::from_u8_lossy(50));
        assert_eq!(FilterCutoff(120).control_value(), U7::from_u8_lossy(107));

        let q = FilterQ(5);
        assert_eq!(q.control_function(), ControlFunction(U7::from_u8_lossy(3)));
        assert_eq!(q.control_value(), U7::from_u8_lossy(5));
        assert_eq!(FilterQ(20).control_value(), U7::from_u8_lossy(10));
    }

    #[test]
    fn test_reverb_parameters() {
        assert_eq!(ReverbType::Room.control_function(), ControlFunction(U7::from_u8_lossy(5)));
        assert_eq!(ReverbType::Room.control_value(), U7::from_u8_lossy(0));
        assert_eq!(ReverbType::Hall.control_function(), ControlFunction(U7::from_u8_lossy(5)));
        assert_eq!(ReverbType::Hall.control_value(), U7::from_u8_lossy(1));
        assert_eq!(ReverbType::Plate.control_function(), ControlFunction(U7::from_u8_lossy(5)));
        assert_eq!(ReverbType::Plate.control_value(), U7::from_u8_lossy(2));

        let decay = ReverbDecay(80);
        assert_eq!(decay.control_function(), ControlFunction(U7::from_u8_lossy(6)));
        assert_eq!(decay.control_value(), U7::from_u8_lossy(80));
        assert_eq!(ReverbDecay(150).control_value(), U7::from_u8_lossy(100));

        let mix = ReverbMix(40);
        assert_eq!(mix.control_function(), ControlFunction(U7::from_u8_lossy(7)));
        assert_eq!(mix.control_value(), U7::from_u8_lossy(40));
        assert_eq!(ReverbMix(150).control_value(), U7::from_u8_lossy(100));
    }

    #[test]
    fn test_delay_parameters() {
        let decay = DelayDecay(70);
        assert_eq!(decay.control_function(), ControlFunction(U7::from_u8_lossy(9)));
        assert_eq!(decay.control_value(), U7::from_u8_lossy(70));
        assert_eq!(DelayDecay(120).control_value(), U7::from_u8_lossy(100));

        let rate = DelayRate(30);
        assert_eq!(rate.control_function(), ControlFunction(U7::from_u8_lossy(10)));
        assert_eq!(rate.control_value(), U7::from_u8_lossy(30));
        assert_eq!(DelayRate(120).control_value(), U7::from_u8_lossy(100));

        let mix = DelayMix(60);
        assert_eq!(mix.control_function(), ControlFunction(U7::from_u8_lossy(11)));
        assert_eq!(mix.control_value(), U7::from_u8_lossy(60));
        assert_eq!(DelayMix(120).control_value(), U7::from_u8_lossy(100));
    }

    #[test]
    fn test_distortion_parameters() {
        let gain = DistortionGain(55);
        assert_eq!(gain.control_function(), ControlFunction(U7::from_u8_lossy(13)));
        assert_eq!(gain.control_value(), U7::from_u8_lossy(55));
        assert_eq!(DistortionGain(120).control_value(), U7::from_u8_lossy(100));

        let tone = DistortionTone(65);
        assert_eq!(tone.control_function(), ControlFunction(U7::from_u8_lossy(14)));
        assert_eq!(tone.control_value(), U7::from_u8_lossy(65));
        assert_eq!(DistortionTone(120).control_value(), U7::from_u8_lossy(100));

        let level = DistortionLevel(75);
        assert_eq!(level.control_function(), ControlFunction(U7::from_u8_lossy(15)));
        assert_eq!(level.control_value(), U7::from_u8_lossy(75));
        assert_eq!(DistortionLevel(120).control_value(), U7::from_u8_lossy(100));
    }

    #[test]
    fn test_chorus_parameters() {
        let freq = ChorusFreq(45);
        assert_eq!(freq.control_function(), ControlFunction(U7::from_u8_lossy(17)));
        assert_eq!(freq.control_value(), U7::from_u8_lossy(45));
        assert_eq!(ChorusFreq(120).control_value(), U7::from_u8_lossy(100));

        let depth = ChorusDepth(85);
        assert_eq!(depth.control_function(), ControlFunction(U7::from_u8_lossy(18)));
        assert_eq!(depth.control_value(), U7::from_u8_lossy(85));
        assert_eq!(ChorusDepth(120).control_value(), U7::from_u8_lossy(100));

        let mix = ChorusMix(95);
        assert_eq!(mix.control_function(), ControlFunction(U7::from_u8_lossy(19)));
        assert_eq!(mix.control_value(), U7::from_u8_lossy(95));
        assert_eq!(ChorusMix(120).control_value(), U7::from_u8_lossy(100));
    }

    #[test]
    fn test_phaser_parameters() {
        let freq = PhaserFreq(25);
        assert_eq!(freq.control_function(), ControlFunction(U7::from_u8_lossy(21)));
        assert_eq!(freq.control_value(), U7::from_u8_lossy(25));
        assert_eq!(PhaserFreq(120).control_value(), U7::from_u8_lossy(100));

        let depth = PhaserDepth(35);
        assert_eq!(depth.control_function(), ControlFunction(U7::from_u8_lossy(22)));
        assert_eq!(depth.control_value(), U7::from_u8_lossy(35));
        assert_eq!(PhaserDepth(120).control_value(), U7::from_u8_lossy(100));

        let mix = PhaserMix(45);
        assert_eq!(mix.control_function(), ControlFunction(U7::from_u8_lossy(23)));
        assert_eq!(mix.control_value(), U7::from_u8_lossy(45));
        assert_eq!(PhaserMix(120).control_value(), U7::from_u8_lossy(100));
    }
}
