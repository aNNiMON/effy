use strum::VariantArray;
use strum_macros::VariantArray;

use crate::{
    params::SelectableOption,
    visitors::{FFmpegParameter, FFmpegParameterVisitor},
};

#[derive(Debug, VariantArray, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AudioBitrate {
    K4,
    K16,
    K32,
    Auto,
    K64,
    K128,
    K192,
    K256,
    K320,
    K512
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
            AudioBitrate::K4 => "4k",
            AudioBitrate::K16 => "16k",
            AudioBitrate::K32 => "32k",
            AudioBitrate::Auto => "auto",
            AudioBitrate::K64 => "64k",
            AudioBitrate::K128 => "128k",
            AudioBitrate::K192 => "192k",
            AudioBitrate::K256 => "256k",
            AudioBitrate::K320 => "320k",
            AudioBitrate::K512 => "512k",
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
