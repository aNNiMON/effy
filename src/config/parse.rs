use crate::config::ConfigError;

const HARDWARE_ACCELERATION_NAMES: &[&str] = &["nvidia", "intel", "amd", "vaapi", "macos"];

pub(crate) fn hwaccel_options(value: &str) -> Result<Vec<String>, ConfigError> {
    value
        .split(',')
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .map(|name| {
            if HARDWARE_ACCELERATION_NAMES.contains(&name) {
                Ok(name.to_owned())
            } else {
                Err(ConfigError::Other(format!(
                    "Unknown value `{name}` in hide_hwaccel_options; Valid values: {}",
                    HARDWARE_ACCELERATION_NAMES.join(", ")
                )))
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hwaccel_options_normalized() {
        assert_eq!(
            hwaccel_options(" amd, intel, ,vaapi ").unwrap(),
            ["amd", "intel", "vaapi"]
        );
    }

    #[test]
    fn hwaccel_options_rejected_for_unknown_name() {
        let error = hwaccel_options("nvidia,other").unwrap_err();

        assert!(matches!(error, ConfigError::Other(cause) if cause.contains("other")));
    }
}
