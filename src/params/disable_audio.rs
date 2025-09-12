use crate::{
    params::macros::struct_option,
    visitors::{FFmpegParameter, FFmpegParameterVisitor},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DisableAudio {
    pub(crate) value: &'static str,
}

impl DisableAudio {
    pub(crate) const NAME: &'static str = "Disable Audio";
    pub(crate) const OFF: &'static str = "off";
    pub(crate) const ON: &'static str = "on";
    pub(crate) const VARIANTS: [&str; 2] = [Self::OFF, Self::ON];

    pub const fn new(value: &'static str) -> Self {
        DisableAudio { value }
    }

    pub const fn default() -> Self {
        DisableAudio { value: Self::OFF }
    }
}

struct_option!(DisableAudio);

impl FFmpegParameter for DisableAudio {
    fn accept(&self, visitor: &mut dyn FFmpegParameterVisitor) {
        visitor.visit_disable_audio(self);
    }
}
