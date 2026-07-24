//! These channels should all be automatically implemented by the macro!
//!
//! This should be part of the DSL/instrument implementation macro.

use instrumentrs::Parameter;

use crate::{InstrumentError, InstrumentParameter};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, Hash)]
pub enum Channel {
    In(Input),
    Out(Output),
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Hash, Parameter)]
#[cmd("{}")]
pub enum Input {
    #[param("A")]
    InA,
    #[param("B")]
    InB,
    #[param("C")]
    InC,
    #[param("D")]
    InD,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Hash, Parameter)]
#[cmd("{}")]
pub enum Output {
    #[param("1")]
    Out1,
    #[param("2")]
    Out2,
    #[param("3")]
    Out3,
    #[param("4")]
    Out4,
}

impl InstrumentParameter<String> for Channel {
    fn to_writable(&self) -> String {
        match self {
            Channel::In(i) => i.to_writable(),
            Channel::Out(o) => o.to_writable(),
        }
    }

    fn try_from_writable(val: String) -> Result<Self, InstrumentError> {
        match val.trim() {
            "A" | "B" | "C" | "D" => Ok(Channel::In(Input::try_from_writable(val)?)),
            "1" | "2" | "3" | "4" => Ok(Channel::Out(Output::try_from_writable(val)?)),
            _ => Err(InstrumentError::BadInstrumentResponseString { msg: val }),
        }
    }
}
