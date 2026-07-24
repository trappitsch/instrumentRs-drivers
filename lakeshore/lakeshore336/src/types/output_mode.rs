//! Types for setting/getting output mode.

use instrumentrs::Parameter;

use crate::{Input, InstrumentError, InstrumentParameter};

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

impl InstrumentParameter<String> for OutputModeSetup {
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
#[derive(Debug, Clone, Copy, PartialEq, Parameter)]
#[cmd("{}")]
pub enum OutputMode {
    #[param("0")]
    Off,
    #[param("1")]
    ClosedLoopPID,
    #[param("2")]
    Zone,
    #[param("3")]
    OpenLoop,
    #[param("4")]
    MonitorOut,
    #[param("5")]
    WarmupSupply,
    #[param("6")]
    Mirroring,
}

/// Sets the status on powering up the instrument.
///
/// This is used for the output mode.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Parameter)]
#[cmd("{}")]
pub enum OnPowerup {
    #[param("0")]
    Disabled,
    #[param("1")]
    Enabled,
}
