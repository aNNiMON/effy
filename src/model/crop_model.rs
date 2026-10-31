use std::fmt::{Display, Formatter};

use tracing::debug;

#[derive(Debug)]
enum CropAxisError {
    EndTooSmall,
    StartExceedsActual(u32),
    StartExceedsEnd(u32),
}

/// Crop parameters
#[derive(Debug, Clone, Default)]
pub(crate) struct CropData {
    pub(crate) x: Option<String>,
    pub(crate) y: Option<String>,
    pub(crate) w: Option<String>,
    pub(crate) h: Option<String>,
}

impl CropData {
    const MIN_SIZE: u32 = 2;
    const MAX_SIZE: u32 = 99999;

    pub(crate) fn is_empty(&self) -> bool {
        self.x.is_none() && self.y.is_none() && self.w.is_none() && self.h.is_none()
    }

    pub(crate) fn validate(
        sx: &str,
        sy: &str,
        sw: &str,
        sh: &str,
        dimensions: (u32, u32),
    ) -> Result<(u32, u32, u32, u32), String> {
        if !sx.is_empty() && !Self::valid_value(sx) {
            return Err("Incorrect x format".to_owned());
        }
        if !sy.is_empty() && !Self::valid_value(sy) {
            return Err("Incorrect y format".to_owned());
        }
        if !sw.is_empty() && !Self::valid_value(sw) {
            return Err("Incorrect w format".to_owned());
        }
        if !sh.is_empty() && !Self::valid_value(sh) {
            return Err("Incorrect h format".to_owned());
        }

        debug!(x=?sx, y=?sy, w=?sw, h=?sh, "Crop validate");

        let x_axis = Self::validate_axis(sx, sw, dimensions.0);
        if let Err(e) = x_axis {
            return match e {
                CropAxisError::EndTooSmall => Err("Width must be at least 2".to_owned()),
                CropAxisError::StartExceedsActual(s) => {
                    Err(format!("X must not exceed iw-2 ({s})"))
                }
                CropAxisError::StartExceedsEnd(s) => {
                    Err(format!("X must not exceed iw-w (or set w<={s})"))
                }
            };
        }
        let y_axis = Self::validate_axis(sy, sh, dimensions.1);
        if let Err(e) = y_axis {
            return match e {
                CropAxisError::EndTooSmall => Err("Height must be at least 2".to_owned()),
                CropAxisError::StartExceedsActual(s) => {
                    Err(format!("Y must not exceed ih-2 ({s})"))
                }
                CropAxisError::StartExceedsEnd(s) => {
                    Err(format!("Y must not exceed ih-h (or set h<={s})"))
                }
            };
        }
        let (x, w) = x_axis.unwrap();
        let (y, h) = y_axis.unwrap();
        Ok((x, y, w, h))
    }

    pub(crate) fn valid_value(value: &str) -> bool {
        value.len() <= 5 && value.chars().all(|c| c.is_ascii_digit())
    }

    /// Validates axis (x, w) or (y, h)
    fn validate_axis(start: &str, end: &str, actual: u32) -> Result<(u32, u32), CropAxisError> {
        if actual < Self::MIN_SIZE {
            return Self::validate_axis_limited(start, end);
        }
        match (start, end) {
            ("", "") => Ok((0, actual)),
            ("", ww) => {
                // width = 2..clamp(iw, w)
                // x = iw/2 - w/2
                let pw = ww.parse::<u32>().unwrap_or(actual);
                if pw < Self::MIN_SIZE {
                    return Err(CropAxisError::EndTooSmall);
                }
                let w = pw.clamp(Self::MIN_SIZE, actual);
                let x = (actual / 2).saturating_sub(w / 2);
                Ok((x, w))
            }
            (xx, "") => {
                // x = 0..(iw-2)
                // width = iw - x
                let px = xx.parse::<u32>().unwrap_or(0);
                if px > actual.saturating_sub(Self::MIN_SIZE) {
                    return Err(CropAxisError::StartExceedsActual(
                        actual.saturating_sub(Self::MIN_SIZE),
                    ));
                }
                let x = px.clamp(0, actual.saturating_sub(Self::MIN_SIZE));
                let w = actual.saturating_sub(x);
                Ok((x, w))
            }
            (xx, ww) => {
                // width = 2..clamp(iw, w)
                // x = 0..(iw-2)
                let pw = ww.parse::<u32>().unwrap_or(actual);
                if pw < Self::MIN_SIZE {
                    return Err(CropAxisError::EndTooSmall);
                }
                let w = pw.clamp(Self::MIN_SIZE, actual);
                let px = xx.parse::<u32>().unwrap_or(0);
                if px > actual.saturating_sub(Self::MIN_SIZE) {
                    return Err(CropAxisError::StartExceedsActual(
                        actual.saturating_sub(Self::MIN_SIZE),
                    ));
                }
                if px.saturating_add(w) > actual {
                    return Err(CropAxisError::StartExceedsEnd(actual.saturating_sub(px)));
                }
                let x = px.clamp(0, actual.saturating_sub(w));
                Ok((x, w))
            }
        }
    }

    fn validate_axis_limited(start: &str, end: &str) -> Result<(u32, u32), CropAxisError> {
        match (start, end) {
            ("", "") => Ok((0, 0)), // 0, iw
            ("", ww) => {
                // width = 2..99999
                let pw = ww.parse::<u32>().unwrap_or(Self::MAX_SIZE);
                if pw < Self::MIN_SIZE {
                    return Err(CropAxisError::EndTooSmall);
                }
                Ok((0, pw))
            }
            (_, "") => Err(CropAxisError::EndTooSmall),
            (xx, ww) => {
                // width = 2..clamp(iw, w)
                // x = 0..(iw-2)
                let pw = ww.parse::<u32>().unwrap_or(Self::MAX_SIZE);
                if pw < Self::MIN_SIZE {
                    return Err(CropAxisError::EndTooSmall);
                }
                let px = xx.parse::<u32>().unwrap_or(0);
                Ok((px, pw))
            }
        }
    }

    pub(crate) fn parse(&self) -> (String, String, String, String) {
        let (x, w) = Self::parse_axis(
            self.x.as_deref().unwrap_or(""),
            self.w.as_deref().unwrap_or(""),
            "iw",
        );
        let (y, h) = Self::parse_axis(
            self.y.as_deref().unwrap_or(""),
            self.h.as_deref().unwrap_or(""),
            "ih",
        );
        (x, y, w, h)
    }

    /// Parses axis (x, w) or (y, h) to ffmpeg crop parameters, using iw/ih variables
    fn parse_axis(start: &str, end: &str, actual_var: &str) -> (String, String) {
        match (start, end) {
            ("", "") => ("0".to_owned(), actual_var.to_owned()),
            ("", ww) => {
                // iw/2-w/2, w
                let pw = ww.parse::<u32>().unwrap_or(0).max(2);
                (format!("{actual_var}/2-{pw}/2"), pw.to_string())
            }
            (xx, "") => {
                // x, iw-x
                let px = xx.parse::<u32>().unwrap_or(0);
                (px.to_string(), format!("{actual_var}-{px}"))
            }
            (xx, ww) => {
                // x, w
                let pw = ww.parse::<u32>().unwrap_or(0).max(2);
                let px = xx.parse::<u32>().unwrap_or(0);
                (px.to_string(), pw.to_string())
            }
        }
    }
}

impl Display for CropData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{},{}x{}",
            self.x.as_deref().unwrap_or("0"),
            self.y.as_deref().unwrap_or("0"),
            self.w.as_deref().unwrap_or("W"),
            self.h.as_deref().unwrap_or("H"),
        )
    }
}
