use crate::{
    params::macros::struct_option,
    visitors::{FFmpegParameter, FFmpegParameterVisitor},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AudioBitrate {
    pub(crate) value: String,
}

impl AudioBitrate {
    pub(crate) const NAME: &'static str = "Audio Bitrate";
    pub(crate) const DEFAULT: &'static str = "auto";
    pub(crate) const VARIANTS: [&str; 10] = [
        "4k", "16k", "32k", "auto", "64k", "128k", "192k", "256k", "320k", "512k",
    ];

    pub const fn new(value: String) -> Self {
        AudioBitrate { value }
    }

    pub fn default() -> Self {
        AudioBitrate {
            value: Self::DEFAULT.into(),
        }
    }
}

struct_option!(AudioBitrate);

impl FFmpegParameter for AudioBitrate {
    fn accept(&self, visitor: &mut dyn FFmpegParameterVisitor) {
        visitor.visit_audio_bitrate(self);
    }
}
