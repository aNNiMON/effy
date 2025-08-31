use strum::VariantArray;
use strum_macros::VariantArray;

use crate::{
    params::SelectableOption,
    visitors::{FFmpegParameter, FFmpegParameterVisitor},
};

#[derive(Debug, VariantArray, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AudioPitch {
    P0_60,
    P0_80,
    P0_90,
    P1_00,
    P1_15,
    P1_25,
    P1_50,
    P2_00,
}

impl SelectableOption for AudioPitch {
    fn variants(&self) -> &'static [Self]
    where
        Self: Sized,
    {
        AudioPitch::VARIANTS
    }

    fn as_str(&self) -> &'static str {
        match self {
            AudioPitch::P0_60 => "0.6",
            AudioPitch::P0_80 => "0.8",
            AudioPitch::P0_90 => "0.9",
            AudioPitch::P1_00 => "1",
            AudioPitch::P1_15 => "1.15",
            AudioPitch::P1_25 => "1.25",
            AudioPitch::P1_50 => "1.5",
            AudioPitch::P2_00 => "2",
        }
    }

    fn describe_self(&self) -> &'static str {
        "Audio Pitch"
    }
}

impl FFmpegParameter for AudioPitch {
    fn accept(&self, visitor: &mut dyn FFmpegParameterVisitor) {
        visitor.visit_audio_pitch(self);
    }
}
