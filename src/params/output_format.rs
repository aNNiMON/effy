use tracing::debug;

use crate::{
    info::Info,
    params::{Parameter, ParameterData, PresetParameter, SelectOption},
    visitors::{CommandBuilder, VisitorContext},
};

pub(crate) struct OutputFormat;

impl OutputFormat {
    pub(crate) const ID: &'static str = "output";
    pub(crate) const NAME: &'static str = "Output";

    const AUDIO_TYPES: [&'static str; 4] = ["mp3", "wav", "flac", "ogg"];

    pub fn new_parameter(info: &Info, source_ext: &str) -> Parameter {
        Parameter::new(
            Self::ID,
            Self::NAME,
            Self::get_parameter_data(info, source_ext),
        )
        .with_order(120)
    }

    pub(crate) fn is_audio(ext: &str) -> bool {
        Self::AUDIO_TYPES.contains(&ext)
    }

    fn get_parameter_data(info: &Info, ext: &str) -> ParameterData {
        let mut selected_index = 0;
        let mut options = Vec::new();
        if info.has_video() {
            options.push("mp4");
        }
        if info.has_audio() {
            options.extend(Self::AUDIO_TYPES);
        }
        if !options.contains(&ext) {
            selected_index = options.len();
            options.push(ext);
        }
        ParameterData::Select {
            options: options.into_iter().map(SelectOption::from).collect(),
            selected_index,
        }
    }

    pub(crate) fn toggle_audio_formats(param: &mut Parameter, new_state: bool) {
        if let ParameterData::Select {
            options,
            selected_index,
        } = &mut param.data
        {
            options
                .iter_mut()
                .filter(|o| Self::is_audio(&o.value))
                .for_each(|option| {
                    option.available = new_state;
                });

            // Shift to next available option if current is not available
            // Or disable entire param if no available options left
            if let Some(current) = options.get(*selected_index)
                && !current.available
            {
                if let Some(new_index) = options.iter().position(|o| o.available) {
                    *selected_index = new_index;
                } else {
                    param.enabled = false;
                }
            }
        }
    }

    pub fn build_command(cb: &mut CommandBuilder, data: &ParameterData) {
        if let ParameterData::Select {
            options,
            selected_index,
        } = data
            && let Some(option) = options.get(*selected_index)
        {
            debug!(value = option.value, "build_command");
            cb.ext.clone_from(&option.value);
        }
    }
}

impl<'a> PresetParameter<'a> for OutputFormat {
    fn apply_preset(_ctx: &VisitorContext, data: &mut ParameterData, preset_value: &str) {
        Self::set_parameter_value(data, preset_value);
    }

    fn save_preset(_ctx: &VisitorContext, data: &'a ParameterData) -> Option<String> {
        if let ParameterData::Select {
            options,
            selected_index,
        } = data
            && let Some(option) = options.get(*selected_index)
        {
            Some(option.value.clone())
        } else {
            None
        }
    }
}
