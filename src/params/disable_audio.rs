use strum::VariantArray;
use strum_macros::VariantArray;

use crate::{
    params::SelectableOption,
    visitors::{FFmpegParameter, FFmpegParameterVisitor},
};

#[derive(Debug, VariantArray, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DisableAudio {
    On,
    Off,
}

impl SelectableOption for DisableAudio {
    fn variants(&self) -> &'static [Self]
    where
        Self: Sized,
    {
        DisableAudio::VARIANTS
    }

    fn as_str(&self) -> &'static str {
        match self {
            DisableAudio::On => "on",
            DisableAudio::Off => "off",
        }
    }

    fn describe_self(&self) -> &'static str {
        "Disable Audio"
    }
}

impl FFmpegParameter for DisableAudio {
    fn accept(&self, visitor: &mut dyn FFmpegParameterVisitor) {
        visitor.visit_disable_audio(self);
    }
}
