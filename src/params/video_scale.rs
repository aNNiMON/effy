use crate::{
    params::macros::struct_option,
    visitors::{FFmpegParameter, FFmpegParameterVisitor},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VideoScale {
    pub(crate) value: String,
}

impl VideoScale {
    pub(crate) const NAME: &'static str = "Video Scale";
    pub(crate) const DEFAULT: &'static str = "original";
    pub(crate) const VARIANTS: [&str; 7] =
        ["144p", "240p", "360p", "original", "480p", "720p", "1080p"];

    pub const fn new(value: String) -> Self {
        VideoScale { value }
    }

    pub fn default() -> Self {
        VideoScale {
            value: Self::DEFAULT.into(),
        }
    }
}

struct_option!(VideoScale);

impl FFmpegParameter for VideoScale {
    fn accept(&self, visitor: &mut dyn FFmpegParameterVisitor) {
        visitor.visit_video_scale(self);
    }
}
