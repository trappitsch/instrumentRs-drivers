//! Provide a properly configured serial interface for the SMR HRR chiller series.
//!
//! This module is only available when the "serialport" feature is activated.

use std::time::Duration;

use serialport::{DataBits, Parity, SerialPortBuilder, StopBits};

/// Baud rates (in bps) supported by the chiller.
pub enum BaudRate {
    /// 9600 bps.
    Bps9600,
    /// 19200 bps.
    Bps19200,
}

impl From<BaudRate> for u32 {
    fn from(value: BaudRate) -> Self {
        match value {
            BaudRate::Bps9600 => 9600,
            BaudRate::Bps19200 => 19200,
        }
    }
}

/// Get a new [`serialport::SerialPortBuilder`] configured for use with a SMC HRR chiller.
///
/// Example:
///
/// ```no_run
/// use smc_hrr::serial_interface::{new_serial_interface, BaudRate};
///
/// let interface = new_serial_interface("/dev/ttyACM0", BaudRate::Bps19200).open().unwrap();
/// ```
///
/// Arguments:
/// - `path`: Path/name of the port, e.g., "/dev/ttyACM0", "COM5".
/// - `baud`: Baud rate as [`BaudRate`].
pub fn new_serial_interface(path: &str, baud: BaudRate) -> SerialPortBuilder {
    serialport::new(path, baud.into())
        .data_bits(DataBits::Eight)
        .stop_bits(StopBits::One)
        .parity(Parity::Even)
        .timeout(Duration::from_secs(3))
}
