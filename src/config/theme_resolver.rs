use crate::ui::Theme;
use std::path::Path;

/// Resolves user-defined themes or overrides built-in themes.
pub struct ThemeResolver;

pub(crate) const BUILTIN_THEMES: &[(&str, &str)] = &[
    ("default", include_str!("../assets/theme-default.toml")),
    ("hacky", include_str!("../assets/theme-hacky.toml")),
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
        if theme_path.exists()
            && let Ok(theme_content) = std::fs::read_to_string(&theme_path)
            && let Ok(theme) = toml::from_str::<toml::Table>(&theme_content)
        {
            // Merge user-defined theme into the source
            source.extend(theme);
        } else if built_in_theme.is_none() {
            return Err(format!("Theme {name} not found"));
        }

        toml::Value::Table(source)
            .try_into()
            .map_err(|_| "Failed to convert theme into the expected format".to_owned())
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
}
