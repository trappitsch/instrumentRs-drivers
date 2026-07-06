//! These channels should all be automatically implemented by the macro!
//!
//! This should be part of the DSL/instrument implementation macro.

use instrumentrs2::InstrumentRsError;

use crate::Parameter;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Hash)]
pub enum Channel {
    In(Input),
    Out(Output),
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum Input {
    InA,
    InB,
    InC,
    InD,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum Output {
    Out1,
    Out2,
    Out3,
    Out4,
}

impl Parameter<String> for Input {
    fn to_writable(&self) -> String {
        match self {
            Input::InA => String::from("A"),
            Input::InB => String::from("B"),
            Input::InC => String::from("C"),
            Input::InD => String::from("D"),
        }
    }

    fn try_from_writable(val: String) -> Result<Self, instrumentrs2::InstrumentRsError> {
        match val.trim() {
            "A" => Ok(Input::InA),
            "B" => Ok(Input::InB),
            "C" => Ok(Input::InC),
            "D" => Ok(Input::InD),
            _ => Err(InstrumentRsError::BadInstrumentResponseString { msg: val }),
        }
    }
}

impl Parameter<String> for Output {
    fn to_writable(&self) -> String {
        match self {
            Output::Out1 => String::from("1"),
            Output::Out2 => String::from("2"),
            Output::Out3 => String::from("3"),
            Output::Out4 => String::from("4"),
        }
    }

    fn try_from_writable(val: String) -> Result<Self, InstrumentRsError> {
        match val.trim() {
            "1" => Ok(Output::Out1),
            "2" => Ok(Output::Out2),
            "3" => Ok(Output::Out3),
            "4" => Ok(Output::Out4),
            _ => Err(InstrumentRsError::BadInstrumentResponseString { msg: val }),
        }
    }
}

impl Parameter<String> for Channel {
    fn to_writable(&self) -> String {
        match self {
            Channel::In(i) => i.to_writable(),
            Channel::Out(o) => o.to_writable(),
        }
    }

    fn try_from_writable(val: String) -> Result<Self, InstrumentRsError> {
        match val.trim() {
            "A" | "B" | "C" | "D" => Ok(Channel::In(Input::try_from_writable(val)?)),
            "1" | "2" | "3" | "4" => Ok(Channel::Out(Output::try_from_writable(val)?)),
            _ => Err(InstrumentRsError::BadInstrumentResponseString { msg: val }),
        }
    }
}
