use std::sync::{Arc, mpsc::Sender};

use crate::model::{
    AppEvent, CustomSelectData, InputConstraints, TrimData, ValidationCallback, ValueFormatter,
};

#[derive(Debug, Clone)]
pub(crate) struct SelectOption {
    pub(crate) name: String,
    pub(crate) value: String,
}

impl From<&str> for SelectOption {
    fn from(value: &str) -> Self {
        SelectOption {
            name: value.to_owned(),
            value: value.to_owned(),
        }
    }
}

impl From<(&str, &str)> for SelectOption {
    fn from((name, value): (&str, &str)) -> Self {
        SelectOption {
            name: name.to_owned(),
            value: value.to_owned(),
        }
    }
}

impl SelectOption {
    pub(crate) fn from_slice(values: &[&str]) -> Vec<Self> {
        values.iter().map(|&v| v.into()).collect()
    }

    pub(crate) fn from_pairs(names: &[(&str, &str)]) -> Vec<Self> {
        names.iter().map(|&v| v.into()).collect()
    }
}

pub(crate) enum ParameterData {
    Select {
        options: Vec<SelectOption>,
        selected_index: usize,
    },
    CustomSelect {
        options: Vec<SelectOption>,
        selected_index: usize,
        value: String,
        constraints: InputConstraints,
        validator: ValidationCallback,
        formatter: Option<ValueFormatter>,
    },
    Toggle {
        value: bool,
    },
    Trim(TrimData),
}

pub(crate) struct Parameter {
    pub(crate) id: &'static str,
    pub(crate) name: String,
    pub(crate) enabled: bool,
    pub(crate) order: usize,
    pub(crate) data: ParameterData,
}

impl Parameter {
    pub(crate) fn new(id: &'static str, name: &str, data: ParameterData) -> Self {
        Parameter {
            id,
            name: name.to_owned(),
            enabled: true,
            order: 0,
            data,
        }
    }

    pub(crate) fn with_order(mut self, order: usize) -> Self {
        self.order = order;
        self
    }

    pub(crate) fn toggle_prev(&mut self, event_sender: &Sender<AppEvent>) {
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
                if !options.is_empty() {
                    *selected_index = if *selected_index == 0 {
                        options.len() - 1
                    } else {
                        *selected_index - 1
                    };
                }
            }
            ParameterData::CustomSelect {
                options,
                selected_index,
                value,
                ..
            } => {
                if !options.is_empty() {
                    *selected_index = if *selected_index == 0 {
                        options.len() - 1
                    } else {
                        *selected_index - 1
                    };
                    value.clone_from(&options[*selected_index].value);
                }
            }
            ParameterData::Trim(_) => self.open_modal(event_sender),
        }
    }

    pub(crate) fn toggle_next(&mut self, event_sender: &Sender<AppEvent>) {
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
                if !options.is_empty() {
                    *selected_index = if *selected_index >= options.len() - 1 {
                        0
                    } else {
                        *selected_index + 1
                    };
                }
            }
            ParameterData::CustomSelect {
                options,
                selected_index,
                value,
                ..
            } => {
                if !options.is_empty() {
                    *selected_index = if *selected_index >= options.len() - 1 {
                        0
                    } else {
                        *selected_index + 1
                    };
                    value.clone_from(&options[*selected_index].value);
                }
            }
            ParameterData::Trim(_) => self.open_modal(event_sender),
        }
    }

    pub(crate) fn describe_value(&self) -> String {
        match &self.data {
            ParameterData::Toggle { value } => if *value { "on" } else { "off" }.to_owned(),
            ParameterData::Select {
                options,
                selected_index,
            } => options
                .get(*selected_index)
                .map_or_else(String::new, |option| option.name.clone()),
            ParameterData::CustomSelect {
                value, formatter, ..
            } => {
                if let Some(fmt) = formatter {
                    fmt(value)
                } else {
                    value.clone()
                }
            }
            ParameterData::Trim(data) => {
                format!(
                    "{}{}..{} {}",
                    if data.precise { "!" } else { "~" },
                    data.ss.as_deref().unwrap_or("start"),
                    data.to.as_deref().unwrap_or("end"),
                    if data.use_to { "(to)" } else { "(duration)" },
                )
            }
        }
    }

    pub(crate) fn describe(&self) -> String {
        format!("{}: {}", &self.name, self.describe_value())
    }

    pub(crate) fn open_modal(&self, event_sender: &Sender<AppEvent>) {
        if !self.enabled {
            return;
        }
        match &self.data {
            ParameterData::CustomSelect {
                value,
                constraints,
                validator,
                ..
            } => {
                let _ = event_sender.send(AppEvent::OpenCustomSelectModal(CustomSelectData {
                    name: Arc::from(self.name.clone()), // todo: global param name to Arc
                    value: value.clone(),
                    constraints: *constraints,
                    validator: Arc::clone(validator),
                }));
            }
            ParameterData::Trim(data) => {
                let _ = event_sender.send(AppEvent::OpenTrimModal(data.clone()));
            }
            _ => {}
        }
    }
}

impl Default for Parameter {
    fn default() -> Self {
        Parameter {
            id: "",
            name: String::new(),
            enabled: false,
            order: 0,
            data: ParameterData::Toggle { value: false },
        }
    }
}

impl ParameterData {
    pub(crate) fn is_editable(&self) -> bool {
        matches!(
            self,
            ParameterData::CustomSelect { .. } | ParameterData::Trim { .. }
        )
    }
}

pub trait PresetParameter {
    fn apply_preset(data: &mut ParameterData, preset_value: &str);
    fn save_preset<'a>(data: &'a ParameterData) -> Option<&'a str>;

    fn set_parameter_value(data: &mut ParameterData, new_value: &str) {
        match data {
            ParameterData::Toggle { value } => {
                *value = new_value == "true" || new_value == "1";
            }
            ParameterData::CustomSelect { value, .. } => {
                new_value.clone_into(value);
            }
            ParameterData::Select {
                options,
                selected_index,
            } => {
                for (i, option) in options.iter().enumerate() {
                    if option.value == new_value {
                        *selected_index = i;
                        break;
                    }
                }
            }
            _ => {}
        }
    }
}
