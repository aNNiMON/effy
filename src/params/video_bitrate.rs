use strum::VariantArray;
use strum_macros::VariantArray;

use crate::{
    params::SelectableOption,
    visitors::{FFmpegParameter, FFmpegParameterVisitor},
};

#[derive(Debug, VariantArray, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VideoBitrate {
    K16,
    K32,
    Auto,
    K64,
    K128,
    K256,
    K512,
    M1,
    M2,
    M4,
    M8,
    M16,
}

impl SelectableOption for VideoBitrate {
    fn variants(&self) -> &'static [Self]
    where
        Self: Sized,
    {
        VideoBitrate::VARIANTS
    }

    fn as_str(&self) -> &'static str {
        match self {
            VideoBitrate::K16 => "16k",
            VideoBitrate::K32 => "32k",
            VideoBitrate::Auto => "auto",
            VideoBitrate::K64 => "64k",
            VideoBitrate::K128 => "128k",
            VideoBitrate::K256 => "256k",
            VideoBitrate::K512 => "512k",
            VideoBitrate::M1 => "1M",
            VideoBitrate::M2 => "2M",
            VideoBitrate::M4 => "4M",
            VideoBitrate::M8 => "8M",
            VideoBitrate::M16 => "16M",
        }
    }

    fn describe_self(&self) -> &'static str {
        "Video Bitrate"
    }
}

impl FFmpegParameter for VideoBitrate {
    fn accept(&self, visitor: &mut dyn FFmpegParameterVisitor) {
        visitor.visit_video_bitrate(self);
    }
}
