pub use {
    channel::{Channel, Input, Output},
    instrument::{InstrumentParameter, Lakeshore336},
    instrumentrs::InstrumentError,
};

pub mod channel;
pub mod instrument;
mod transport;
pub mod types;

// feature serialport
#[cfg(feature = "serialport")]
mod serial_interface;
#[cfg(feature = "serialport")]
pub use serial_interface::new_serialport;
