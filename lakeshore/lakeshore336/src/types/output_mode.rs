//! Types for setting/getting output mode.

use crate::{Input, InstrumentError, Parameter};

/// Output mode setup.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OutputModeSetup {
    output_mode: OutputMode,
    input_to_follow: Option<Input>,
    on_powerup: OnPowerup,
}

impl OutputModeSetup {
    /// Create a ne output mode setup configuration.
    pub fn new(
        output_mode: OutputMode,
        input_to_follow: Option<Input>,
        on_powerup: OnPowerup,
    ) -> Self {
        Self {
            output_mode,
            input_to_follow,
            on_powerup,
        }
    }
}

impl Parameter<String> for OutputModeSetup {
    fn to_writable(&self) -> String {
        let i2f = match self.input_to_follow {
            None => "0",
            Some(inp) => match inp {
                Input::InA => "1",
                Input::InB => "2",
                Input::InC => "3",
                Input::InD => "4",
            },
        };
        format!(
            "{},{},{}",
            self.output_mode.to_writable(),
            i2f,
            self.on_powerup.to_writable()
        )
    }

    fn try_from_writable(val: String) -> Result<Self, InstrumentError> {
        let split_vals = val.trim().split(',').collect::<Vec<&str>>();
        if split_vals.len() != 3 {
            return Err(InstrumentError::BadInstrumentResponseString { msg: val });
        }

        let output_mode = OutputMode::try_from_writable(split_vals[0].into())?;
        let input_to_follow = match split_vals[1] {
            "0" => None,
            "1" => Some(Input::InA),
            "2" => Some(Input::InB),
            "3" => Some(Input::InC),
            "4" => Some(Input::InD),
            _ => return Err(InstrumentError::BadInstrumentResponseString { msg: val }),
        };
        let on_powerup = OnPowerup::try_from_writable(split_vals[2].into())?;

        Ok(Self {
            output_mode,
            input_to_follow,
            on_powerup,
        })
    }
}

/// The output modes that are available.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputMode {
    Off,
    ClosedLoopPID,
    Zone,
    OpenLoop,
    MonitorOut,
    WarmupSupply,
    Mirroring,
}

impl Parameter<String> for OutputMode {
    fn to_writable(&self) -> String {
        match self {
            OutputMode::Off => "0".to_string(),
            OutputMode::ClosedLoopPID => "1".to_string(),
            OutputMode::Zone => "2".to_string(),
            OutputMode::OpenLoop => "3".to_string(),
            OutputMode::MonitorOut => "4".to_string(),
            OutputMode::WarmupSupply => "5".to_string(),
            OutputMode::Mirroring => "6".to_string(),
        }
    }

    fn try_from_writable(val: String) -> Result<Self, instrumentrs2::InstrumentError> {
        match val.trim() {
            "0" => Ok(OutputMode::Off),
            "1" => Ok(OutputMode::ClosedLoopPID),
            "2" => Ok(OutputMode::Zone),
            "3" => Ok(OutputMode::OpenLoop),
            "4" => Ok(OutputMode::MonitorOut),
            "5" => Ok(OutputMode::WarmupSupply),
            "6" => Ok(OutputMode::Mirroring),
            _ => Err(InstrumentError::BadInstrumentResponseString { msg: val }),
        }
    }
}

/// Sets the status on powering up the instrument.
///
/// This is used for the output mode.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OnPowerup {
    Disabled,
    Enabled,
}

impl Parameter<String> for OnPowerup {
    fn to_writable(&self) -> String {
        match self {
            OnPowerup::Disabled => "0".to_string(),
            OnPowerup::Enabled => "1".to_string(),
        }
    }

    fn try_from_writable(val: String) -> Result<Self, InstrumentError> {
        match val.trim() {
            "0" => Ok(OnPowerup::Disabled),
            "1" => Ok(OnPowerup::Enabled),
            _ => Err(InstrumentError::BadInstrumentResponseString { msg: val }),
        }
    }
}
