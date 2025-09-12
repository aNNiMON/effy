use crate::{
    params::macros::struct_option,
    visitors::{FFmpegParameter, FFmpegParameterVisitor},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VideoBitrate {
    pub(crate) value: &'static str,
}

impl VideoBitrate {
    pub(crate) const NAME: &'static str = "Video Bitrate";
    pub(crate) const DEFAULT: &'static str = "auto";
    pub(crate) const VARIANTS: [&str; 12] = [
        "16k", "32k", "auto", "64k", "128k", "256k", "512k", "1M", "2M", "4M", "8M", "16M",
    ];

    pub const fn new(value: &'static str) -> Self {
        VideoBitrate { value }
    }

    pub const fn default() -> Self {
        VideoBitrate {
            value: Self::DEFAULT,
        }
    }
}

struct_option!(VideoBitrate);

impl FFmpegParameter for VideoBitrate {
    fn accept(&self, visitor: &mut dyn FFmpegParameterVisitor) {
        visitor.visit_video_bitrate(self);
    }
}
