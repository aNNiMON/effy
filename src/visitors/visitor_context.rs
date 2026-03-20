use crate::info::Info;

/// Shared context for visitors

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct VisitorContext {
    // from input info
    pub(crate) input_duration: Option<f64>,
}

impl VisitorContext {
    pub(crate) fn new(info: &Info) -> Self {
        Self {
            input_duration: info.get_duration(),
        }
    }
}
