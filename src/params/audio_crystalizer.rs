use strum::VariantArray;
use strum_macros::VariantArray;

use crate::{
    params::SelectableOption,
    visitors::{FFmpegParameter, FFmpegParameterVisitor},
};

#[derive(Debug, VariantArray, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AudioCrystalizer {
    M8,
    M4,
    M2,
    Zero,
    P2,
    P4,
    P8,
}

impl SelectableOption for AudioCrystalizer {
    fn variants(&self) -> &'static [Self]
    where
        Self: Sized,
    {
        AudioCrystalizer::VARIANTS
    }

    fn as_str(&self) -> &'static str {
        match self {
            AudioCrystalizer::M8 => "-8",
            AudioCrystalizer::M4 => "-4",
            AudioCrystalizer::M2 => "-2",
            AudioCrystalizer::Zero => "0",
            AudioCrystalizer::P2 => "2",
            AudioCrystalizer::P4 => "4",
            AudioCrystalizer::P8 => "8",
        }
    }

    fn describe_self(&self) -> &'static str {
        "Audio Crystalizer"
    }
}

impl FFmpegParameter for AudioCrystalizer {
    fn accept(&self, visitor: &mut dyn FFmpegParameterVisitor) {
        visitor.visit_audio_crystalizer(self);
    }
}
