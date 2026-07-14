//! Implement types for outputs.

use measurements::Current;

use crate::{InstrumentError, Parameter};

/// Heater setup.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HeaterSetup {
    resistance: HeaterResistance,
    max_current: HeaterMaxOutputCurrent,
    display: HeaterOutputDisplay,
}

impl HeaterSetup {
    /// Try to create a new heater setup from the provided resistance, max_current, and display values.
    ///
    /// TODO: Check if the limitations remarked in the manual need to be in the constructor or if
    /// instrument automatically limits them. Add a remark here on how this is handled.
    ///
    /// Errors:
    /// - The maximum current setup is a
    pub fn try_new(
        resistance: HeaterResistance,
        max_current: HeaterMaxOutputCurrent,
        display: HeaterOutputDisplay,
    ) -> Result<Self, InstrumentError> {
        max_current.valid_user_current()?;
        Ok(Self {
            resistance,
            max_current,
            display,
        })
    }
}

impl Parameter<String> for HeaterSetup {
    fn to_writable(&self) -> String {
        format!(
            "{},{},{}",
            self.resistance.to_writable(),
            self.max_current.to_writable(),
            self.display.to_writable()
        )
    }

    fn try_from_writable(val: String) -> Result<Self, InstrumentError> {
        let split_vals = val.trim().split(',').collect::<Vec<&str>>();
        if split_vals.len() != 4 {
            return Err(InstrumentError::BadInstrumentResponseString { msg: val });
        }

        let resistance = HeaterResistance::try_from_writable(split_vals[0].into())?;
        let max_current = HeaterMaxOutputCurrent::try_from_writable(format!(
            "{},{}",
            split_vals[1], split_vals[2]
        ))?;
        let display = HeaterOutputDisplay::try_from_writable(split_vals[3].into())?;

        Ok(Self {
            resistance,
            max_current,
            display,
        })
    }
}

/// Heater resistance setting.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HeaterResistance {
    /// 25 Ohm.
    R25Ohm,
    /// 50 Ohm.
    R50Ohm,
}

impl Parameter<String> for HeaterResistance {
    fn to_writable(&self) -> String {
        match self {
            HeaterResistance::R25Ohm => String::from("1"),
            HeaterResistance::R50Ohm => String::from("2"),
        }
    }

    fn try_from_writable(val: String) -> Result<Self, InstrumentError> {
        match val.trim() {
            "1" => Ok(HeaterResistance::R25Ohm),
            "2" => Ok(HeaterResistance::R50Ohm),
            _ => Err(InstrumentError::BadInstrumentResponseString { msg: val }),
        }
    }
}

/// Maximum heater output current.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HeaterMaxOutputCurrent {
    /// User specified output current, must be between 0 A and 2 A.
    User(Current),
    /// 0.707 A.
    C0p707A,
    /// 1 A.
    C1A,
    /// 1.141 A.
    C1p141A,
    /// 2 A.
    C2A,
}

impl HeaterMaxOutputCurrent {
    /// Check if the provided user current is valid.
    fn valid_user_current(&self) -> Result<(), InstrumentError> {
        if let HeaterMaxOutputCurrent::User(user_current) = self
            && (*user_current < Current::from_amperes(0.)
                || *user_current > Current::from_amperes(2.0))
        {
            return Err(InstrumentError::UnitfulValueOutOfRange {
                unit: "A".into(),
                val: user_current.as_amperes(),
                val_min: 0.0,
                val_max: 2.0,
            });
        }
        Ok(())
    }
}

impl Parameter<String> for HeaterMaxOutputCurrent {
    fn to_writable(&self) -> String {
        match self {
            HeaterMaxOutputCurrent::User(c) => format!("0,+{:.3}", c.as_amperes()),
            HeaterMaxOutputCurrent::C0p707A => "1,+0.707".into(),
            HeaterMaxOutputCurrent::C1A => "2,+1.000".into(),
            HeaterMaxOutputCurrent::C1p141A => "3,+1.141".into(),
            HeaterMaxOutputCurrent::C2A => "4,+2.000".into(),
        }
    }

    fn try_from_writable(val: String) -> Result<Self, InstrumentError> {
        let val_split = val.trim().split(",").collect::<Vec<&str>>();
        if val_split.len() < 2 {
            return Err(InstrumentError::BadInstrumentResponseString { msg: val });
        }

        match val_split[0] {
            "0" => {
                let flt_rec = val_split[1].trim_start_matches("+").parse::<f64>()?;
                Ok(HeaterMaxOutputCurrent::User(Current::from_amperes(flt_rec)))
            }
            "1" => Ok(HeaterMaxOutputCurrent::C0p707A),
            "2" => Ok(HeaterMaxOutputCurrent::C1A),
            "3" => Ok(HeaterMaxOutputCurrent::C1p141A),
            "4" => Ok(HeaterMaxOutputCurrent::C2A),
            _ => Err(InstrumentError::BadInstrumentResponseString { msg: val }),
        }
    }
}

/// Heater output display units.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HeaterOutputDisplay {
    /// Show heater output current on display.
    Current,
    /// Show heater output power on display.
    Power,
}

impl Parameter<String> for HeaterOutputDisplay {
    fn to_writable(&self) -> String {
        match self {
            HeaterOutputDisplay::Current => "1".into(),
            HeaterOutputDisplay::Power => "2".into(),
        }
    }

    fn try_from_writable(val: String) -> Result<Self, InstrumentError> {
        match val.trim() {
            "1" => Ok(HeaterOutputDisplay::Current),
            "2" => Ok(HeaterOutputDisplay::Power),
            _ => Err(InstrumentError::BadInstrumentResponseString { msg: val }),
        }
    }
}

/// Range for the heater output.
///
/// Note that for Outputs 2 and 3, only Off and LowOrOn are valid.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HeaterRange {
    /// Heater output is off.
    Off,
    /// Heater output is low (output Out1 and Out2) or on (output Out3 or Out4)
    LowOrOn,
    /// Heater output is medium (only Out1 and Out2).
    Medium,
    /// Heater output is high (only Out1 and Out2).
    High,
}

impl Parameter<String> for HeaterRange {
    fn to_writable(&self) -> String {
        match self {
            HeaterRange::Off => "0".to_string(),
            HeaterRange::LowOrOn => "1".to_string(),
            HeaterRange::Medium => "2".to_string(),
            HeaterRange::High => "3".to_string(),
        }
    }

    fn try_from_writable(val: String) -> Result<Self, InstrumentError> {
        match val.trim() {
            "0" => Ok(HeaterRange::Off),
            "1" => Ok(HeaterRange::LowOrOn),
            "2" => Ok(HeaterRange::Medium),
            "3" => Ok(HeaterRange::High),
            _ => Err(InstrumentError::BadInstrumentResponseString { msg: val }),
        }
    }
}
