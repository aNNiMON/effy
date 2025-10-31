use std::sync::mpsc::Sender;

use crate::model::{AppEvent, TrimData};

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
    Toggle {
        value: bool,
    },
    Trim(TrimData),
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
            name: id.to_owned(),
            enabled: true,
            data,
        }
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
                if options.is_empty() {
                    return;
                }
                if *selected_index == 0 {
                    *selected_index = options.len() - 1;
                } else {
                    *selected_index -= 1;
                }
            }
            ParameterData::Trim(data) => {
                let _ = event_sender.send(AppEvent::OpenTrimModal(data.clone()));
            }
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
                if options.is_empty() {
                    return;
                }
                if *selected_index >= options.len() - 1 {
                    *selected_index = 0;
                } else {
                    *selected_index += 1;
                }
            }
            ParameterData::Trim(data) => {
                let _ = event_sender.send(AppEvent::OpenTrimModal(data.clone()));
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
            ParameterData::Trim(data) => {
                format!(
                    "{}{}..{}{}",
                    if data.precise { "!" } else { "~" },
                    data.ss.clone().unwrap_or("start".to_string()),
                    if data.use_to { "to: " } else { "duration: " },
                    data.to.clone().unwrap_or("end".to_string())
                )
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
