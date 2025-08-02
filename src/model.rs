#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Pane {
    Info,
    Params,
    Config,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AudioBitrate {
    K32,
    K80,
    K128,
    K192,
    K256,
    Auto,
}

impl AudioBitrate {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            AudioBitrate::K32 => "32k",
            AudioBitrate::K80 => "80k",
            AudioBitrate::K128 => "128k",
            AudioBitrate::K192 => "192k",
            AudioBitrate::K256 => "256k",
            AudioBitrate::Auto => "auto",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VideoBitrate {
    K768,
    M1,
    M2,
    M4,
    Auto,
}

impl VideoBitrate {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            VideoBitrate::K768 => "768k",
            VideoBitrate::M1 => "1M",
            VideoBitrate::M2 => "2M",
            VideoBitrate::M4 => "4M",
            VideoBitrate::Auto => "auto",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Param {
    DisableAudio(bool),
    AudioBitrate(AudioBitrate),
    VideoBitrate(VideoBitrate),
}
