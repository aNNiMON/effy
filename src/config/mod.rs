mod parse;

use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    fs, io,
    path::{Path, PathBuf},
};

use serde::Deserialize;

pub(crate) const _DEFAULT_CONFIG: &str = include_str!("../assets/config-default.toml");

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct RawConfig {
    hide_hwaccel_options: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Config {
    pub(crate) hide_hwaccel_options: Vec<String>,
}

impl Config {
    pub(crate) fn load(path: Option<&Path>) -> Result<Self, ConfigError> {
        let path = path.map(Path::to_owned).map_or_else(Self::path, Ok)?;
        let contents = match fs::read_to_string(&path) {
            Ok(contents) => contents,
            Err(source) if source.kind() == io::ErrorKind::NotFound => {
                return Ok(Self::default());
            }
            Err(source) => return Err(ConfigError::Read { path, source }),
        };
        let raw: RawConfig = toml::from_str(&contents).map_err(|source| ConfigError::Parse {
            path: path.clone(),
            source,
        })?;

        Ok(Self {
            hide_hwaccel_options: parse::hwaccel_options(&raw.hide_hwaccel_options)?,
        })
    }

    fn path() -> Result<PathBuf, ConfigError> {
        let config_dir = std::env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .filter(|path| path.is_absolute())
            .or_else(dirs::config_dir)
            .or_else(|| dirs::home_dir().map(|home| home.join(".config")))
            .ok_or(ConfigError::PathUnavailable)?;
        let app_name = env!("CARGO_PKG_NAME");

        Ok(config_dir.join(app_name).join(format!("{app_name}.toml")))
    }
}

#[derive(Debug)]
pub(crate) enum ConfigError {
    PathUnavailable,
    Read {
        path: PathBuf,
        source: io::Error,
    },
    Parse {
        path: PathBuf,
        source: toml::de::Error,
    },
    Other(String),
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::PathUnavailable => write!(f, "could not determine the user config directory"),
            Self::Read { path, source } => {
                write!(f, "could not read config {}: {source}", path.display())
            }
            Self::Parse { path, source } => {
                write!(f, "could not parse config {}: {source}", path.display())
            }
            Self::Other(cause) => write!(f, "{}", cause),
        }
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Read { source, .. } => Some(source),
            Self::Parse { source, .. } => Some(source),
            Self::PathUnavailable | Self::Other(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::{Path, PathBuf},
        sync::atomic::{AtomicU64, Ordering},
    };

    use super::{_DEFAULT_CONFIG, Config, ConfigError};

    static NEXT_TEMP_DIR: AtomicU64 = AtomicU64::new(0);

    struct TempDir(PathBuf);

    impl TempDir {
        fn new() -> Self {
            let id = NEXT_TEMP_DIR.fetch_add(1, Ordering::Relaxed);
            let path =
                std::env::temp_dir().join(format!("effy-config-{}-{id}", std::process::id()));
            fs::create_dir(&path).expect("create test directory");
            Self(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn should_use_default_config_on_missing_file() {
        let temp = TempDir::new();
        let path = temp.path().join("missing").join("effy.toml");

        assert_eq!(Config::load(Some(&path)).unwrap(), Config::default());
        assert!(!path.exists());
        assert!(!path.parent().unwrap().exists());
    }

    #[test]
    fn should_use_default_config_on_empty_file() {
        let temp = TempDir::new();
        let path = temp.path().join("effy.toml");
        fs::write(&path, "").unwrap();

        assert_eq!(Config::load(Some(&path)).unwrap(), Config::default());
    }

    #[test]
    fn should_fail_on_corrupted_file() {
        let temp = TempDir::new();
        let path = temp.path().join("effy.toml");
        fs::write(&path, "public static void main(String[] args) {}").unwrap();

        let error = Config::load(Some(&path)).unwrap_err();

        assert!(matches!(error, ConfigError::Parse { .. }));
        assert!(error.to_string().contains(path.to_str().unwrap()));
    }

    // Full options config

    #[test]
    fn should_load_default_config() {
        let temp = TempDir::new();
        let path = temp.path().join("effy.toml");
        fs::write(&path, _DEFAULT_CONFIG).unwrap();

        assert_eq!(Config::load(Some(&path)).unwrap(), Config::default());
    }
}
