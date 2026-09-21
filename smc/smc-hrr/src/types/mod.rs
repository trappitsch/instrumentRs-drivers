//! Types for the SMC HRR chillers.

pub use comms::SerialRemoteInstruction;
pub use control::{OperationStatus, SetTemperature};
pub use flags::{AlarmFlags, StatusFlag};
pub use reads::{DischargePressure, DischargeTemperature, ElectricConductivity, FlowRate};
pub use slave_address::SlaveAddress;

mod comms;
mod control;
mod flags;
mod reads;
mod slave_address;
