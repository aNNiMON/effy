use crate::{
    params::macros::struct_option,
    visitors::{FFmpegParameter, FFmpegParameterVisitor},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SpeedFactor {
    pub(crate) value: String,
}

impl SpeedFactor {
    pub(crate) const NAME: &'static str = "Speed";
    pub(crate) const DEFAULT: &'static str = "1";
    pub(crate) const VARIANTS: [&str; 13] = [
        "0.5", "0.75", "0.8", "0.9", "1", "1.25", "1.4", "1.5", "1.6", "1.8", "2", "2.5", "3",
    ];

    pub const fn new(value: String) -> Self {
        SpeedFactor { value }
    }

    pub fn default() -> Self {
        SpeedFactor {
            value: Self::DEFAULT.into(),
        }
    }
}

struct_option!(SpeedFactor);

impl FFmpegParameter for SpeedFactor {
    fn accept(&self, visitor: &mut dyn FFmpegParameterVisitor) {
        visitor.visit_speed_factor(self);
    }
}
