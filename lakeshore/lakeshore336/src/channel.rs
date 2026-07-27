//! These channels should all be automatically implemented by the macro!
//!
//! This should be part of the DSL/instrument implementation macro.

use instrumentrs::Parameter;

use crate::InstrumentParameter;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, Hash)]
pub enum Channel {
    In(Input),
    Out(Output),
}

impl Channel {
    pub fn inner_to_writable(&self) -> String {
        match self {
            Channel::In(i) => i.to_writable(),
            Channel::Out(o) => o.to_writable(),
        }
    }
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
