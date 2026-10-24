use std::collections::HashMap;

use crate::{
    params::*,
    visitors::{PRESET_SEPARATOR, PRESET_VALUE_SEPARATOR, ParameterVisitor, VisitorContext},
};

/// Apply preset

#[derive(Default)]
pub(crate) struct PresetApplier<'a> {
    ctx: VisitorContext,
    preset_map: HashMap<&'a str, &'a str>,
}

impl<'a> PresetApplier<'a> {
    pub(crate) fn new(ctx: VisitorContext, preset: &'a str) -> Self {
        let preset_map = split_preset(preset)
            .into_iter()
            .filter_map(|p| p.split_once(PRESET_VALUE_SEPARATOR))
            .collect::<HashMap<&str, &str>>();
        Self { ctx, preset_map }
    }
}

impl ParameterVisitor for PresetApplier<'_> {
    fn visit_trim(&mut self, data: &mut ParameterData) {
        if let Some(preset_value) = self.preset_map.get(Trim::ID) {
            Trim::apply_preset(&self.ctx, data, preset_value);
        }
    }

    fn visit_disable_audio(&mut self, data: &mut ParameterData) {
        if let Some(preset_value) = self.preset_map.get(DisableAudio::ID) {
            DisableAudio::apply_preset(&self.ctx, data, preset_value);
        }
    }

    fn visit_audio_bitrate(&mut self, data: &mut ParameterData) {
        if let Some(preset_value) = self.preset_map.get(AudioBitrate::ID) {
            AudioBitrate::apply_preset(&self.ctx, data, preset_value);
        }
    }

    fn visit_audio_crystalizer(&mut self, data: &mut ParameterData) {
        if let Some(preset_value) = self.preset_map.get(AudioCrystalizer::ID) {
            AudioCrystalizer::apply_preset(&self.ctx, data, preset_value);
        }
    }

    fn visit_audio_volume(&mut self, data: &mut ParameterData) {
        if let Some(preset_value) = self.preset_map.get(AudioVolume::ID) {
            AudioVolume::apply_preset(&self.ctx, data, preset_value);
        }
    }

    fn visit_audio_pitch(&mut self, data: &mut ParameterData) {
        if let Some(preset_value) = self.preset_map.get(AudioPitch::ID) {
            AudioPitch::apply_preset(&self.ctx, data, preset_value);
        }
    }

    fn visit_speed_factor(&mut self, data: &mut ParameterData) {
        if let Some(preset_value) = self.preset_map.get(SpeedFactor::ID) {
            SpeedFactor::apply_preset(&self.ctx, data, preset_value);
        }
    }

    fn visit_crop(&mut self, data: &mut ParameterData) {
        if let Some(preset_value) = self.preset_map.get(Crop::ID) {
            Crop::apply_preset(&self.ctx, data, preset_value);
        }
    }

    fn visit_video_bitrate(&mut self, data: &mut ParameterData) {
        if let Some(preset_value) = self.preset_map.get(VideoBitrate::ID) {
            VideoBitrate::apply_preset(&self.ctx, data, preset_value);
        }
    }

    fn visit_video_frame_rate(&mut self, data: &mut ParameterData) {
        if let Some(preset_value) = self.preset_map.get(VideoFrameRate::ID) {
            VideoFrameRate::apply_preset(&self.ctx, data, preset_value);
        }
    }

    fn visit_video_scale(&mut self, data: &mut ParameterData) {
        if let Some(preset_value) = self.preset_map.get(VideoScale::ID) {
            VideoScale::apply_preset(&self.ctx, data, preset_value);
        }
    }

    fn visit_lut(&mut self, data: &mut ParameterData) {
        if let Some(preset_value) = self.preset_map.get(Lut::ID) {
            Lut::apply_preset(&self.ctx, data, &unescape_preset_value(preset_value));
        }
    }

    fn visit_hardware_acceleration(&mut self, data: &mut ParameterData) {
        if let Some(preset_value) = self.preset_map.get(HardwareAcceleration::ID) {
            HardwareAcceleration::apply_preset(&self.ctx, data, preset_value);
        }
    }

    fn visit_output_format(&mut self, data: &mut ParameterData) {
        if let Some(preset_value) = self.preset_map.get(OutputFormat::ID) {
            OutputFormat::apply_preset(&self.ctx, data, preset_value);
        }
    }

    fn visit_last(&mut self) {}
}

fn split_preset(preset: &str) -> Vec<&str> {
    let mut start = 0;
    let mut escaped = false;
    let mut parts = Vec::new();

    for (index, ch) in preset.char_indices() {
        if escaped {
            escaped = false;
        } else if ch == '\\' {
            escaped = true;
        } else if ch == PRESET_SEPARATOR {
            parts.push(&preset[start..index]);
            start = index + ch.len_utf8();
        }
    }

    parts.push(&preset[start..]);
    parts
}

fn unescape_preset_value(value: &str) -> String {
    let mut unescaped = String::with_capacity(value.len());
    let mut chars = value.chars();

    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some(escaped @ ('\\' | PRESET_SEPARATOR)) => unescaped.push(escaped),
                Some(next) => {
                    unescaped.push(ch);
                    unescaped.push(next);
                }
                None => unescaped.push(ch),
            }
        } else {
            unescaped.push(ch);
        }
    }

    unescaped
}

#[cfg(test)]
mod tests {
    use super::PresetApplier;
    use crate::visitors::VisitorContext;

    #[test]
    fn parses_escaped_preset() {
        let applier = PresetApplier::new(
            VisitorContext::default(),
            r"lut=folder\\aa'bb\;cc.cube;output=mp4",
        );

        assert_eq!(
            applier.preset_map.get("lut"),
            Some(&r"folder\\aa'bb\;cc.cube")
        );
        assert_eq!(applier.preset_map.get("output"), Some(&"mp4"));
    }
}
