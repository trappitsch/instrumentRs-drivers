//! Holds types for controlling the chiller.

use std::fmt;

use measurements::Temperature;

use crate::InstrumentParameter;

/// Operation status.
#[derive(Debug, PartialEq)]
pub enum OperationStatus {
    /// Chiller is running.
    Run,
    /// Chiller is stopped.
    Stop,
}

impl InstrumentParameter<Vec<u8>> for OperationStatus {
    fn to_writable(&self) -> Vec<u8> {
        match self {
            Self::Run => vec![0x00, 0x01],
            Self::Stop => vec![0x00, 0x00],
        }
    }

    fn try_from_writable(val: Vec<u8>) -> Result<Self, instrumentrs::InstrumentError> {
        if val.len() != 2 {
            return Err(instrumentrs::InstrumentError::BadInstrumentResponseVecU8 { msg: val });
        }

        match val[1] & 0x01 {
            0x01 => Ok(Self::Run),
            0x00 => Ok(Self::Stop),
            _ => Err(instrumentrs::InstrumentError::BadInstrumentResponseVecU8 { msg: val }),
        }
    }
}

/// Set temperature of the fluid.
///
/// This is the set temperature of the circulating fluid. This value can bet set or requested.
/// You can convert a `SetTemperature` into and from a `measurements::Temperature` value directly.
/// If the value is out of range when turning a `measurements::Temperature` into a `SetTemperature`,
/// the min/max possible value is automatically chosen.
///
/// TODO: Doc setting it from a temperature.
#[derive(Debug, PartialEq, PartialOrd)]
pub struct SetTemperature {
    value: i16,
}

impl From<SetTemperature> for Temperature {
    fn from(value: SetTemperature) -> Self {
        Temperature::from_celsius(value.value as f64 / 10.)
    }
}

impl From<&SetTemperature> for Temperature {
    fn from(value: &SetTemperature) -> Self {
        Temperature::from_celsius(value.value as f64 / 10.)
    }
}

impl From<Temperature> for SetTemperature {
    fn from(value: Temperature) -> Self {
        let val_c = (value.as_celsius() * 10.).round();

        Self {
            value: val_c as i16,
        }
    }
}

impl fmt::Display for SetTemperature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t: Temperature = self.into();
        write!(f, "{t}")
    }
}

impl InstrumentParameter<Vec<u8>> for SetTemperature {
    fn to_writable(&self) -> Vec<u8> {
        Vec::from(self.value.to_be_bytes())
    }

    fn try_from_writable(val: Vec<u8>) -> Result<Self, instrumentrs::InstrumentError> {
        if val.len() != 2 {
            return Err(instrumentrs::InstrumentError::BadInstrumentResponseVecU8 { msg: val });
        }
        let val = [val[0], val[1]];

        Ok(Self {
            value: i16::from_be_bytes(val),
        })
    }
}

#[cfg(test)]
mod tests {
    use measurements::test_utils::assert_almost_eq;

    use super::*;

    #[test]
    fn set_temperature_as_temperature() {
        let st = SetTemperature { value: i16::MIN };
        let t: Temperature = st.into();
        assert_almost_eq(t.as_celsius(), -3276.8);

        let st = SetTemperature { value: i16::MAX };
        let t: Temperature = st.into();
        assert_almost_eq(t.as_celsius(), 3276.7);

        let st = SetTemperature { value: 0 };
        let t: Temperature = st.into();
        assert_eq!(t.as_celsius(), 0.0);
    }
}
