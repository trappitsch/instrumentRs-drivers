//! Types that are used for communication settings.

use crate::InstrumentParameter;

#[derive(Debug, PartialEq)]
pub enum SerialRemoteInstruction {
    None,
    LocalDio,
    Serial,
}

impl InstrumentParameter<Vec<u8>> for SerialRemoteInstruction {
    fn to_writable(&self) -> Vec<u8> {
        match self {
            Self::None => vec![0x00, 0x00],
            Self::LocalDio => vec![0x00, 0x10],
            Self::Serial => vec![0x00, 0x30],
        }
    }

    fn try_from_writable(val: Vec<u8>) -> Result<Self, instrumentrs::InstrumentError> {
        if val.len() != 2 {
            return Err(instrumentrs::InstrumentError::BadInstrumentResponseVecU8 { msg: val });
        }

        match val[1] & 0x30 {
            0x00 => Ok(Self::None),
            0x10 => Ok(Self::LocalDio),
            0x30 => Ok(Self::Serial),
            _ => Err(instrumentrs::InstrumentError::BadInstrumentResponseVecU8 { msg: val }),
        }
    }
}
