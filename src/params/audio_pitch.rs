use crate::{
    params::macros::struct_option,
    visitors::{FFmpegParameter, FFmpegParameterVisitor},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AudioPitch {
    pub(crate) value: String,
}

impl AudioPitch {
    pub(crate) const NAME: &'static str = "Audio Pitch";
    pub(crate) const DEFAULT: &'static str = "1";
    pub(crate) const VARIANTS: [&str; 8] = ["0.6", "0.8", "0.9", "1", "1.15", "1.25", "1.5", "2"];

    pub fn new(value: String) -> Self {
        AudioPitch { value }
    }

    pub fn default() -> Self {
        AudioPitch {
            value: Self::DEFAULT.into(),
        }
    }
}

struct_option!(AudioPitch);

impl FFmpegParameter for AudioPitch {
    fn accept(&self, visitor: &mut dyn FFmpegParameterVisitor) {
        visitor.visit_audio_pitch(self);
    }
}
