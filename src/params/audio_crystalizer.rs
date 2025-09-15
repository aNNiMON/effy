use crate::{
    params::macros::struct_option,
    visitors::{FFmpegParameter, FFmpegParameterVisitor},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AudioCrystalizer {
    pub(crate) value: String,
}

impl AudioCrystalizer {
    pub(crate) const NAME: &'static str = "Audio Crystalizer";
    pub(crate) const DEFAULT: &'static str = "0";
    pub(crate) const VARIANTS: [&str; 7] = ["-8", "-4", "-2", "0", "2", "4", "8"];

    pub const fn new(value: String) -> Self {
        AudioCrystalizer { value }
    }

    pub fn default() -> Self {
        AudioCrystalizer {
            value: Self::DEFAULT.into(),
        }
    }
}

struct_option!(AudioCrystalizer);

impl FFmpegParameter for AudioCrystalizer {
    fn accept(&self, visitor: &mut dyn FFmpegParameterVisitor) {
        visitor.visit_audio_crystalizer(self);
    }
}
