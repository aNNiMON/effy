use strum::VariantArray;
use strum_macros::VariantArray;

use crate::{
    params::SelectableOption,
    visitors::{FFmpegParameter, FFmpegParameterVisitor},
};

#[derive(Debug, VariantArray, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VideoFrameRate {
    Fps5,
    Fps10,
    Fps15,
    Fps20,
    Original,
    Fps30,
    Fps45,
    Fps60,
}

impl SelectableOption for VideoFrameRate {
    fn variants(&self) -> &'static [Self]
    where
        Self: Sized,
    {
        VideoFrameRate::VARIANTS
    }

    fn as_str(&self) -> &'static str {
        match self {
            VideoFrameRate::Fps5 => "5",
            VideoFrameRate::Fps10 => "10",
            VideoFrameRate::Fps15 => "15",
            VideoFrameRate::Fps20 => "20",
            VideoFrameRate::Original => "original",
            VideoFrameRate::Fps30 => "30",
            VideoFrameRate::Fps45 => "45",
            VideoFrameRate::Fps60 => "60",
        }
    }

    fn describe_self(&self) -> &'static str {
        "Video Frame Rate"
    }
}

impl FFmpegParameter for VideoFrameRate {
    fn accept(&self, visitor: &mut dyn FFmpegParameterVisitor) {
        visitor.visit_video_frame_rate(self);
    }
}
