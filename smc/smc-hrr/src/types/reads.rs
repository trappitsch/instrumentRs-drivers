//! Impl new types for reads of chiller conditions.

use std::fmt;

use measurements::{Pressure, Temperature};

use crate::InstrumentParameter;

/// Circulating fluid discharge temperature.
///
/// This parameter can only be read from the instrument. To use it further, you can turn it into a
/// `measurements::Temperature`.
#[derive(Debug, PartialEq, PartialOrd)]
pub struct DischargeTemperature {
    value: i16,
}

impl From<DischargeTemperature> for Temperature {
    fn from(value: DischargeTemperature) -> Temperature {
        Temperature::from_celsius(value.value as f64 / 10.)
    }
}

impl From<&DischargeTemperature> for Temperature {
    fn from(value: &DischargeTemperature) -> Temperature {
        Temperature::from_celsius(value.value as f64 / 10.)
    }
}

impl fmt::Display for DischargeTemperature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t: Temperature = self.into();
        write!(f, "{t}")
    }
}

impl InstrumentParameter<Vec<u8>> for DischargeTemperature {
    fn to_writable(&self) -> Vec<u8> {
        unreachable!("This is a read only variable. To writable should never be called.");
    }

    fn try_from_writable(val: Vec<u8>) -> Result<Self, instrumentrs::InstrumentError> {
        if val.len() != 2 {
            return Err(instrumentrs::InstrumentError::BadInstrumentResponseVecU8 { msg: val });
        }
        let value = i16::from_be_bytes([val[0], val[1]]);

        // range check
        if !(-3276..=3276).contains(&value) {
            return Err(instrumentrs::InstrumentError::BadInstrumentResponseVecU8 { msg: val });
        };

        Ok(Self { value })
    }
}

/// Circulating fluid flow rate.
///
/// This parameter can only be read from the instrument.
#[derive(Debug, PartialEq, PartialOrd)]
pub struct FlowRate {
    value: i16,
}

impl FlowRate {
    /// Get the numeric value in liters per minute.
    pub fn as_lpm(&self) -> f64 {
        self.value as f64 / 10.
    }
}

impl fmt::Display for FlowRate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} lpm", self.as_lpm())
    }
}

impl InstrumentParameter<Vec<u8>> for FlowRate {
    fn to_writable(&self) -> Vec<u8> {
        unreachable!("This is a read only variable. To writable should never be called.");
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

/// Circulating fluid discharge pressure.
///
/// This parameter can only be read from the instrument. To use it further, you can turn it into a
/// `measurements::Pressure`.
#[derive(Debug, PartialEq, PartialOrd)]
pub struct DischargePressure {
    value: i16,
}

impl DischargePressure {
    fn as_megapascals(&self) -> f64 {
        self.value as f64 * 0.001
    }
}

impl From<DischargePressure> for Pressure {
    fn from(value: DischargePressure) -> Pressure {
        Pressure::from_pascals(value.as_megapascals() * 1_000_000.0)
    }
}

impl From<&DischargePressure> for Pressure {
    fn from(value: &DischargePressure) -> Pressure {
        Pressure::from_pascals(value.as_megapascals() * 1_000_000.0)
    }
}

impl fmt::Display for DischargePressure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t: Pressure = self.into();
        write!(f, "{t}")
    }
}

impl InstrumentParameter<Vec<u8>> for DischargePressure {
    fn to_writable(&self) -> Vec<u8> {
        unreachable!("This is a read only variable. To writable should never be called.");
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

/// Circulating fluid electric conductivity.
///
/// This parameter can only be read from the instrument.
#[derive(Debug, PartialEq, PartialOrd)]
pub struct ElectricConductivity {
    value: i16,
}

impl ElectricConductivity {
    /// Get the numeric value in liters per minute.
    pub fn as_microsievert_per_cm(&self) -> f64 {
        self.value as f64 / 10.
    }
}

impl fmt::Display for ElectricConductivity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} µS/cm", self.as_microsievert_per_cm())
    }
}

impl InstrumentParameter<Vec<u8>> for ElectricConductivity {
    fn to_writable(&self) -> Vec<u8> {
        unreachable!("This is a read only variable. To writable should never be called.");
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
