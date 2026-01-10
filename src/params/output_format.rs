use crate::{
    info::Info,
    params::{Parameter, ParameterData, PresetParameter, SelectOption},
    visitors::CommandBuilder,
};

pub(crate) struct OutputFormat;

impl OutputFormat {
    pub(crate) const ID: &'static str = "output";
    pub(crate) const NAME: &'static str = "Output";

    pub fn new_parameter(info: &Info, source_ext: &str) -> Parameter {
        Parameter::new(
            Self::ID,
            Self::NAME,
            Self::get_parameter_data(info, source_ext),
        )
        .with_order(999999)
    }

    pub(crate) fn is_audio(ext: &str) -> bool {
        ext == "mp3" || ext == "wav" || ext == "flac" || ext == "ogg"
    }

    fn get_parameter_data(info: &Info, ext: &str) -> ParameterData {
        let mut selected_index = 0;
        let mut options = Vec::new();
        if info.has_video() {
            options.push("mp4");
        }
        if info.has_audio() {
            options.push("mp3");
            options.push("wav");
            options.push("flac");
            options.push("ogg");
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

    pub fn build_command(cb: &mut CommandBuilder, data: &ParameterData) {
        if let ParameterData::Select {
            options,
            selected_index,
        } = data
            && let Some(option) = options.get(*selected_index)
        {
            cb.ext.clone_from(&option.value);
        }
    }
}

impl PresetParameter for OutputFormat {
    fn apply_preset(data: &mut ParameterData, preset_value: &str) {
        Self::set_parameter_value(data, preset_value);
    }

    fn save_preset(data: &ParameterData) -> Option<&str> {
        if let ParameterData::Select {
            options,
            selected_index,
        } = data
            && let Some(option) = options.get(*selected_index)
        {
            Some(&option.value)
        } else {
            None
        }
    }
}
