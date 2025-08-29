use strum::VariantArray;
use strum_macros::VariantArray;

use crate::{
    params::SelectableOption,
    visitors::{FFmpegParameter, FFmpegParameterVisitor},
};

#[derive(Debug, VariantArray, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AudioBitrate {
    K32,
    K80,
    K128,
    K192,
    K256,
    Auto,
}

impl SelectableOption for AudioBitrate {
    fn variants(&self) -> &'static [Self]
    where
        Self: Sized,
    {
        AudioBitrate::VARIANTS
    }

    fn as_str(&self) -> &'static str {
        match self {
            AudioBitrate::K32 => "32k",
            AudioBitrate::K80 => "80k",
            AudioBitrate::K128 => "128k",
            AudioBitrate::K192 => "192k",
            AudioBitrate::K256 => "256k",
            AudioBitrate::Auto => "auto",
        }
    }

    fn describe_self(&self) -> &'static str {
        "Audio Bitrate"
    }
}

impl FFmpegParameter for AudioBitrate {
    fn accept(&self, visitor: &mut dyn FFmpegParameterVisitor) {
        visitor.visit_audio_bitrate(self);
    }
}
