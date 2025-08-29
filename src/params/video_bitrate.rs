use strum::VariantArray;
use strum_macros::VariantArray;

use crate::params::SelectableOption;

#[derive(Debug, VariantArray, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VideoBitrate {
    K768,
    M1,
    M2,
    M4,
    Auto,
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
            VideoBitrate::K768 => "768k",
            VideoBitrate::M1 => "1M",
            VideoBitrate::M2 => "2M",
            VideoBitrate::M4 => "4M",
            VideoBitrate::Auto => "auto",
        }
    }

    fn describe_self(&self) -> &'static str {
        "Video Bitrate"
    }
}
