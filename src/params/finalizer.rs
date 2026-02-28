use tracing::debug;

use crate::visitors::CommandBuilder;

/// Always enabled and always last virtual parameter
/// Does not rely on other parameters that might be disabled by user choice
pub(crate) struct Finalizer;

impl Finalizer {
    pub fn build_command(cb: &mut CommandBuilder) {
        debug!("build_command finalizer");
        if cb.ext == "mp4" || cb.ext == "mov" {
            cb.pre_output_args.push("-movflags".to_owned());
            cb.pre_output_args.push("faststart".to_owned());
        }
    }
}
