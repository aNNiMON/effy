use std::{
    fmt::{Display, Formatter},
    fs, io, iter,
    path::{Path, PathBuf},
};

/// Lut parameters

#[derive(Debug, Clone, Default)]
pub(crate) enum LutFiles {
    #[default]
    NotScanned, // not scanned yet
    Error(&'static str), // no LUT files found
    Files(Vec<PathBuf>),
}

impl LutFiles {
    pub(crate) fn list(&self) -> Option<&[PathBuf]> {
        match self {
            LutFiles::Files(files) => Some(files),
            _ => None,
        }
    }

    pub(crate) fn scan(&mut self, folder: &Path) -> &Self {
        // Scan if not already scanned and rescan on error
        if !matches!(self, LutFiles::Files(_)) {
            let mut files = Vec::new();
            let result = Self::collect_files(folder, &mut files);
            files.sort();
            files.dedup();
            if result.is_err() {
                *self = LutFiles::Error("Could not read the LUT folder.");
            } else if files.is_empty() {
                *self = LutFiles::Error("No .cube, .3dl files found.");
            } else {
                *self = LutFiles::Files(files);
            }
        }
        self
    }

    fn collect_files(folder: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
        for entry in fs::read_dir(folder)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && Self::is_lut(&path) {
                files.push(path);
            }
        }
        Ok(())
    }

    pub(crate) fn is_lut(path: &Path) -> bool {
        path.extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| {
                let supported = ["cube", "3dl", "csp", "dat", "m3d"];
                supported.iter().any(|s| ext.eq_ignore_ascii_case(s))
            })
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct LutData {
    pub(crate) folder: PathBuf,
    pub(crate) files: LutFiles,
    pub(crate) selected: Option<PathBuf>,
}

impl LutData {
    pub(crate) fn selected_index(&self) -> usize {
        self.files
            .list()
            .as_ref()
            .and_then(|files| {
                self.selected
                    .as_ref()
                    .and_then(|selected| files.iter().position(|p| p == selected))
            })
            .map_or(0, |index| index + 1)
    }

    pub(crate) fn select(&mut self, index: usize) {
        self.selected = index.checked_sub(1).and_then(|index| {
            self.files
                .list()
                .and_then(|files| files.get(index))
                .cloned()
        });
    }

    pub(crate) fn toggle_next(&mut self) {
        self.step(true)
    }

    pub(crate) fn toggle_prev(&mut self) {
        self.step(false)
    }

    fn step(&mut self, next: bool) {
        let count = self.files.list().map_or(0, |a| a.len()) + 1;
        let index = self.selected_index();
        self.select(if next {
            (index + 1) % count
        } else {
            (index + count - 1) % count
        });
    }

    pub(crate) fn names(&self) -> Vec<String> {
        iter::once("none".to_owned())
            .chain(self.files.list().map_or(vec![], |files| {
                files
                    .iter()
                    .map(|path| Self::filename(path))
                    .collect::<Vec<_>>()
            }))
            .collect()
    }

    fn filename(path: &Path) -> String {
        path.file_name()
            .unwrap_or(path.as_os_str())
            .to_string_lossy()
            .into_owned()
    }
}

impl Display for LutData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.selected
                .as_deref()
                .map_or_else(|| "none".to_owned(), Self::filename)
        )
    }
}
