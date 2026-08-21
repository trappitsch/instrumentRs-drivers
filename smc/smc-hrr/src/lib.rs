//! A Rust driver to communicate with an SMC HRR chiller.
//!
//! This driver is based on the [`instrumentrs`] framework.
//! The following chiller models are supported:
//!
//! - HRR010-A/W-10-*
//! - HRR012-A/W-10-*
//! - HRR018-A/W-10-*
//! - HRR010-A/W-20-*
//! - HRR012-A/W-20-*
//! - HRR018-A/W-20-*
//! - HRR024-A/W-20-*
//! - HRR030-A/W-20-*
//! - HRR050-A/W-20-*
//! - HRR050-A/W-40-*
//!
//! This driver uses the MODBUS RTU protocol to communicate with the chiller. Make sure that your
//! chiller is set up to this protocol.
//! The driver is interface agnostic. Likely, your chiller is connected via a serial port.
//! If so, you can activate the "serialport" feature and get a simple-to-use function that will return
//! a [`serialport`] interface.
//!
//! ## Example
//!
//! This example requires the "serialport" feature (activated by default):
//!
//! ```no_run
//! use smc_hrr::{
//!     SmcHrr,
//!     serial_interface::{new_serial_interface, BaudRate}
//! };
//!
//! let path = "/dev/ttyUSB0";
//! let baud = BaudRate::Bps19200;
//!
//! let interface = new_serial_interface(path, baud).open().unwrap();
//!
//! let mut chiller = SmcHrr::new(interface);
//!
//! println!("Discharge pressure: {}", chiller.get_discharge_pressure().unwrap());
//! ```
//!
//! ## Available features
//!
//! - **"serialport"**: Provides you with a simple [`serialport`] interface that can directly be
//!   used (see example above). This feature is activated by default.
//! - **"serde"**: Provides serialization/deserialization for all the configuration structs and enums,
//!   allowing to, e.g., store instrument configurations in a text file.

pub mod instrument;
mod transport;
pub mod types;

#[cfg(feature = "serialport")]
pub mod serial_interface;

pub use instrument::{InstrumentParameter, SmcHrr};
pub use instrumentrs::InstrumentError;
