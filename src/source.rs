use std::path::Path;

pub(crate) struct Source {
    // Raw input string, e.g. file path or URL
    pub(crate) input: String,
    pub(crate) source_type: SourceType,
}

#[derive(PartialEq)]
pub(crate) enum SourceType {
    File,
    Url,
}

impl Source {
    pub(crate) fn new(input: String) -> Self {
        let lcinput = input.to_lowercase();
        let source_type = if lcinput.starts_with("http://") || lcinput.starts_with("https://") {
            SourceType::Url
        } else {
            SourceType::File
        };
        Self { input, source_type }
    }

    pub(crate) fn validate(&self) -> Result<(), String> {
        if self.source_type == SourceType::File && std::fs::metadata(&self.input).is_err() {
            return Err(format!("Input file '{}' does not exist", self.input));
        }
        Ok(())
    }

    pub(crate) fn input_folder(&self) -> String {
        match self.source_type {
            SourceType::Url => ".".to_string(),
            SourceType::File => Path::new(&self.input)
                .parent()
                .unwrap_or_else(|| Path::new("."))
                .to_string_lossy()
                .to_string(),
        }
    }

    pub(crate) fn input_name_and_ext(&self) -> (String, String) {
        let path = Path::new(&self.input);
        let filename = path
            .file_stem()
            .unwrap_or_else(|| path.file_name().unwrap())
            .to_string_lossy();
        let ext = path
            .extension()
            .unwrap_or_else(|| "mp4".as_ref())
            .to_string_lossy();
        (filename.to_string(), ext.to_string())
    }
}
