use crate::ui::Theme;
use std::{io::ErrorKind, path::Path};

/// Resolves user-defined themes or overrides built-in themes.
pub struct ThemeResolver;

pub(crate) const BUILTIN_THEMES: &[(&str, &str)] = &[
    ("default", include_str!("../assets/theme-default.toml")),
    ("hacky", include_str!("../assets/theme-hacky.toml")),
    ("bloody", include_str!("../assets/theme-bloody.toml")),
];

impl ThemeResolver {
    pub fn try_resolve(name: &str, config_dir: &Path) -> Result<Theme, String> {
        if !Self::is_valid_name(name) {
            return Err("Theme name contains invalid characters".to_owned());
        }

        // Built-in themes support override
        let built_in_theme = BUILTIN_THEMES.iter().find(|(n, _)| *n == name);
        let mut source: toml::Table = built_in_theme
            // Important to load a default theme anyway, it will be either
            // fully overriden by new theme, or used as fallback (incl. partially)
            .or_else(|| BUILTIN_THEMES.first())
            .and_then(|(_, content)| toml::from_str(content).ok())
            .expect("No themes available");

        // Try to find a user-defined theme file in the config directory
        let theme_path = config_dir.join(format!("theme-{name}.toml"));
        let theme_content = match std::fs::read_to_string(&theme_path) {
            Ok(content) => Some(content),
            Err(error) if error.kind() == ErrorKind::NotFound => None,
            Err(error) => {
                return Err(format!(
                    "Failed to read theme {}: {error}",
                    theme_path.display()
                ));
            }
        };

        if let Some(content) = theme_content {
            let overrides: toml::Table = toml::from_str(&content).map_err(|error| {
                format!("Failed to parse theme {}: {error}", theme_path.display())
            })?;
            source.extend(overrides);
        } else if built_in_theme.is_none() {
            return Err(format!("Theme {name} not found"));
        };

        toml::Value::Table(source).try_into().map_err(|error| {
            format!("Failed to convert theme {name} into the expected format: {error}")
        })
    }

    fn is_valid_name(name: &str) -> bool {
        name.chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_'))
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::{Path, PathBuf},
        sync::atomic::{AtomicU64, Ordering},
    };

    use crate::config::theme_resolver::{BUILTIN_THEMES, ThemeResolver};

    use crate::ui::Theme;

    static NEXT_TEMP_DIR: AtomicU64 = AtomicU64::new(0);

    struct TempDir(PathBuf);

    impl TempDir {
        fn new() -> Self {
            let id = NEXT_TEMP_DIR.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir()
                .join(format!("effy-theme-resolver-{}-{id}", std::process::id()));
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
    fn builtin_themes_deserialize() {
        for (name, content) in BUILTIN_THEMES {
            let _: Theme = toml::from_str(content)
                .unwrap_or_else(|_| panic!("Theme {name} should deserialize correctly"));
        }
    }

    #[test]
    fn custom_theme_names_allow_hyphens_and_underscores() {
        let temp = TempDir::new();

        for name in ["solarized-dark", "my_theme"] {
            fs::write(
                temp.path().join(format!("theme-{name}.toml")),
                "accent = \"red\"",
            )
            .unwrap();

            assert!(ThemeResolver::try_resolve(name, temp.path()).is_ok());
        }
    }

    #[test]
    fn malformed_override_reports_its_path_and_parse_error() {
        let temp = TempDir::new();
        let path = temp.path().join("theme-default.toml");
        fs::write(&path, "accent = [").unwrap();

        let error = ThemeResolver::try_resolve("default", temp.path()).unwrap_err();

        assert!(error.contains(&path.display().to_string()));
        assert!(error.contains("Failed to parse theme"));
    }

    #[test]
    fn unreadable_override_reports_its_path_and_read_error() {
        let temp = TempDir::new();
        let path = temp.path().join("theme-default.toml");
        fs::create_dir(&path).unwrap();

        let error = ThemeResolver::try_resolve("default", temp.path()).unwrap_err();

        assert!(error.contains(&path.display().to_string()));
        assert!(error.contains("Failed to read theme"));
    }

    #[test]
    fn invalid_color_reports_its_field_and_theme_name() {
        let temp = TempDir::new();
        let path = temp.path().join("theme-default.toml");
        fs::write(&path, "accent = \"not-a-color\"").unwrap();

        let error = ThemeResolver::try_resolve("default", temp.path()).unwrap_err();

        assert!(error.contains("Failed to convert theme default"));
        assert!(error.contains("accent"));
    }

    #[test]
    fn unknown_field_reports_its_name_and_theme_name() {
        let temp = TempDir::new();
        let path = temp.path().join("theme-default.toml");
        fs::write(&path, "accnet = \"red\"").unwrap();

        let error = ThemeResolver::try_resolve("default", temp.path()).unwrap_err();

        assert!(error.contains("Failed to convert theme default"));
        assert!(error.contains("accnet"));
        assert!(error.contains("unknown field"));
    }
}
