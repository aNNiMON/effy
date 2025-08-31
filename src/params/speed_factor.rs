use strum::VariantArray;
use strum_macros::VariantArray;

use crate::{
    params::SelectableOption,
    visitors::{FFmpegParameter, FFmpegParameterVisitor},
};

#[derive(Debug, VariantArray, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SpeedFactor {
    X0_50,
    X0_75,
    X0_80,
    X0_90,
    X1_00,
    X1_25,
    X1_40,
    X1_50,
    X1_60,
    X1_80,
    X2_00,
    X2_50,
    X3_00,
}

impl SelectableOption for SpeedFactor {
    fn variants(&self) -> &'static [Self]
    where
        Self: Sized,
    {
        SpeedFactor::VARIANTS
    }

    fn as_str(&self) -> &'static str {
        match self {
            SpeedFactor::X0_50 => "0.5",
            SpeedFactor::X0_75 => "0.75",
            SpeedFactor::X0_80 => "0.8",
            SpeedFactor::X0_90 => "0.9",
            SpeedFactor::X1_00 => "1",
            SpeedFactor::X1_25 => "1.25",
            SpeedFactor::X1_40 => "1.4",
            SpeedFactor::X1_50 => "1.5",
            SpeedFactor::X1_60 => "1.6",
            SpeedFactor::X1_80 => "1.8",
            SpeedFactor::X2_00 => "2",
            SpeedFactor::X2_50 => "2.5",
            SpeedFactor::X3_00 => "3",
        }
    }

    fn describe_self(&self) -> &'static str {
        "Speed"
    }
}

impl FFmpegParameter for SpeedFactor {
    fn accept(&self, visitor: &mut dyn FFmpegParameterVisitor) {
        visitor.visit_speed_factor(self);
    }
}
