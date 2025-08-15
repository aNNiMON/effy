use std::io::{Error, ErrorKind, Read};
use std::process::{Command, Stdio};

pub(crate) fn get_info(input_file: String) -> Result<String, Error> {
    let mut child = match Command::new("ffprobe")
        .args(&[
            "-v",
            "quiet",
            "-of",
            "json=compact=1",
            "-show_format",
            "-show_streams",
        ])
        .arg(&input_file)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(child) => child,
        Err(e) => return Err(e.into()),
    };

    let mut output = String::new();
    if let Some(mut stdout) = child.stdout.take() {
        stdout.read_to_string(&mut output)?;
    }

    let status = child.wait()?;
    if status.success() {
        Ok(output)
    } else {
        Err(Error::new(
            ErrorKind::Other,
            format!("ffprobe exited with status: {}", status),
        ))
    }
}
