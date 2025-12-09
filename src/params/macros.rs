macro_rules! select_option {
    ($data:expr) => {
        if let &ParameterData::Select {
            ref options,
            ref selected_index,
        } = $data
        {
            options.get(*selected_index)
        } else {
            None
        }
    };
}

macro_rules! select_non_default_option {
    ($data:expr) => {
        if let &ParameterData::Select {
            ref options,
            ref selected_index,
        } = $data
            && let Some(option) = options.get(*selected_index)
            && option.value != Self::DEFAULT
        {
            Some(option)
        } else {
            None
        }
    };
}

macro_rules! select_non_default_custom_value {
    ($data:expr) => {
        if let &ParameterData::CustomSelect { ref value, .. } = $data
            && value != Self::DEFAULT
        {
            Some(value)
        } else {
            None
        }
    };
}

pub(crate) use select_non_default_custom_value;
pub(crate) use select_non_default_option;
pub(crate) use select_option;
