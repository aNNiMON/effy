#[derive(Debug, Clone)]
pub(crate) struct SelectOption {
    pub(crate) name: String,
    pub(crate) value: String,
}

impl SelectOption {
    pub(crate) fn from_slice(values: &[&str]) -> Vec<Self> {
        values
            .iter()
            .map(|&value| Self {
                name: value.to_string(),
                value: value.to_string(),
            })
            .collect()
    }

    pub(crate) fn from_pairs(names: &[(&str, &str)]) -> Vec<Self> {
        names
            .iter()
            .map(|(name, value)| Self {
                name: name.to_string(),
                value: value.to_string(),
            })
            .collect()
    }
}

pub(crate) enum ParameterData {
    Select {
        options: Vec<SelectOption>,
        selected_index: usize,
    },
    Toggle {
        value: bool,
    },
}

pub(crate) struct Parameter {
    pub(crate) id: &'static str,
    pub(crate) name: String,
    pub(crate) enabled: bool,
    pub(crate) data: ParameterData,
}

impl Parameter {
    pub(crate) fn new(id: &'static str, data: ParameterData) -> Self {
        Parameter {
            id,
            name: id.to_string(),
            enabled: true,
            data,
        }
    }

    pub(crate) fn toggle_prev(&mut self) {
        if !self.enabled {
            return;
        }
        match &mut self.data {
            ParameterData::Toggle { value } => {
                *value = !*value;
            }
            ParameterData::Select {
                options,
                selected_index,
            } => {
                if options.is_empty() {
                    return;
                }
                if *selected_index == 0 {
                    *selected_index = options.len() - 1;
                } else {
                    *selected_index -= 1;
                }
            }
        }
    }

    pub(crate) fn toggle_next(&mut self) {
        if !self.enabled {
            return;
        }
        match &mut self.data {
            ParameterData::Toggle { value } => {
                *value = !*value;
            }
            ParameterData::Select {
                options,
                selected_index,
            } => {
                if options.is_empty() {
                    return;
                }
                if *selected_index >= options.len() - 1 {
                    *selected_index = 0;
                } else {
                    *selected_index += 1;
                }
            }
        }
    }

    pub(crate) fn describe_value(&self) -> String {
        match &self.data {
            ParameterData::Toggle { value } => if *value { "on" } else { "off" }.to_string(),
            ParameterData::Select {
                options,
                selected_index,
            } => {
                if let Some(option) = options.get(*selected_index) {
                    option.name.clone()
                } else {
                    String::new()
                }
            }
        }
    }

    pub(crate) fn describe(&self) -> String {
        format!("{}: {}", self.name, self.describe_value())
    }
}

impl Default for Parameter {
    fn default() -> Self {
        Parameter {
            id: "",
            name: String::new(),
            enabled: false,
            data: ParameterData::Toggle { value: false },
        }
    }
}
