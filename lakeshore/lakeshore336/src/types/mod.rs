use measurements::Temperature;

use crate::{InstrumentRsError, Parameter};

pub mod heater_setup;
pub mod output_mode;

pub use {
    heater_setup::{HeaterMaxOutputCurrent, HeaterOutputDisplay, HeaterResistance, HeaterSetup},
    output_mode::{OnPowerup, OutputMode, OutputModeSetup},
};

/// Temperature measurement.
impl Parameter<String> for Temperature {
    fn to_writable(&self) -> String {
        format!("{:.1}", self.as_kelvin())
    }

    fn try_from_writable(val: String) -> Result<Self, InstrumentRsError> {
        let val = val.trim().parse::<f64>()?;
        Ok(Temperature::from_kelvin(val))
    }
}

impl Parameter<String> for String {
    fn to_writable(&self) -> String {
        String::from(self)
    }

    fn try_from_writable(val: String) -> Result<String, InstrumentRsError> {
        Ok(String::from(val.trim().trim_matches('"')))
    }
}
