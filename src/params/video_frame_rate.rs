use crate::{
    params::macros::struct_option,
    visitors::{FFmpegParameter, FFmpegParameterVisitor},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VideoFrameRate {
    pub(crate) value: String,
}

impl VideoFrameRate {
    pub(crate) const NAME: &'static str = "Video Frame Rate";
    pub(crate) const DEFAULT: &'static str = "original";
    pub(crate) const VARIANTS: [&str; 8] = ["5", "10", "15", "20", "original", "30", "45", "60"];

    pub const fn new(value: String) -> Self {
        VideoFrameRate { value }
    }

    pub fn default() -> Self {
        VideoFrameRate {
            value: Self::DEFAULT.into(),
        }
    }
}

struct_option!(VideoFrameRate);

impl FFmpegParameter for VideoFrameRate {
    fn accept(&self, visitor: &mut dyn FFmpegParameterVisitor) {
        visitor.visit_video_frame_rate(self);
    }
}
