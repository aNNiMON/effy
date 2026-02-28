use tracing::debug;

use crate::visitors::CommandBuilder;

/// Always enabled and always last virtual parameter
/// Does not rely on other parameters that might be disabled by user choice
pub(crate) struct Finalizer;

impl Finalizer {
    pub fn build_command(_cb: &mut CommandBuilder) {
        debug!("build_command finalizer");
    }
}
