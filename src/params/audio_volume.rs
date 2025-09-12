use crate::{
    params::macros::struct_option,
    visitors::{FFmpegParameter, FFmpegParameterVisitor},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AudioVolume {
    pub(crate) value: &'static str,
}

impl AudioVolume {
    pub(crate) const NAME: &'static str = "Audio Volume";
    pub(crate) const DEFAULT: &'static str = "original";
    pub(crate) const VARIANTS: [&str; 11] = [
        "-15dB", "-10dB", "-5dB", "-2dB", "original", "2dB", "5dB", "10dB", "15dB", "30dB", "50dB",
    ];

    pub const fn new(value: &'static str) -> Self {
        AudioVolume { value }
    }

    pub const fn default() -> Self {
        AudioVolume {
            value: Self::DEFAULT,
        }
    }
}

struct_option!(AudioVolume);

impl FFmpegParameter for AudioVolume {
    fn accept(&self, visitor: &mut dyn FFmpegParameterVisitor) {
        visitor.visit_audio_volume(self);
    }
}
