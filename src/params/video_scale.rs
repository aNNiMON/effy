use strum::VariantArray;
use strum_macros::VariantArray;

use crate::{
    params::SelectableOption,
    visitors::{FFmpegParameter, FFmpegParameterVisitor},
};

#[derive(Debug, VariantArray, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VideoScale {
    K144,
    K240,
    K360,
    Original,
    K480,
    K720,
    K1080,
}

impl SelectableOption for VideoScale {
    fn variants(&self) -> &'static [Self]
    where
        Self: Sized,
    {
        VideoScale::VARIANTS
    }

    fn as_str(&self) -> &'static str {
        match self {
            VideoScale::K144 => "144p",
            VideoScale::K240 => "240p",
            VideoScale::K360 => "360p",
            VideoScale::Original => "original",
            VideoScale::K480 => "480p",
            VideoScale::K720 => "720p",
            VideoScale::K1080 => "1080p",
        }
    }

    fn describe_self(&self) -> &'static str {
        "Video Scale"
    }
}

impl FFmpegParameter for VideoScale {
    fn accept(&self, visitor: &mut dyn FFmpegParameterVisitor) {
        visitor.visit_video_scale(self);
    }
}
