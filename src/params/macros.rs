macro_rules! select_non_default_option {
    ($data:expr) => {
        if let ParameterData::Select {
            options,
            selected_index,
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

pub(crate) use select_non_default_option;
