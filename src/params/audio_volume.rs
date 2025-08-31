use strum::VariantArray;
use strum_macros::VariantArray;

use crate::{
    params::SelectableOption,
    visitors::{FFmpegParameter, FFmpegParameterVisitor},
};

#[derive(Debug, VariantArray, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AudioVolume {
    M15,
    M10,
    M5,
    M2,
    Original,
    P2,
    P5,
    P10,
    P15,
    P30,
    P50,
}

impl SelectableOption for AudioVolume {
    fn variants(&self) -> &'static [Self]
    where
        Self: Sized,
    {
        AudioVolume::VARIANTS
    }

    fn as_str(&self) -> &'static str {
        match self {
            AudioVolume::M15 => "-15dB",
            AudioVolume::M10 => "-10dB",
            AudioVolume::M5 => "-5dB",
            AudioVolume::M2 => "-2dB",
            AudioVolume::Original => "original",
            AudioVolume::P2 => "2dB",
            AudioVolume::P5 => "5dB",
            AudioVolume::P10 => "10dB",
            AudioVolume::P15 => "15dB",
            AudioVolume::P30 => "30dB",
            AudioVolume::P50 => "50dB",
        }
    }

    fn describe_self(&self) -> &'static str {
        "Audio Volume"
    }
}

impl FFmpegParameter for AudioVolume {
    fn accept(&self, visitor: &mut dyn FFmpegParameterVisitor) {
        visitor.visit_audio_volume(self);
    }
}
