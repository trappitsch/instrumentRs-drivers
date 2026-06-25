//! Function to return a pre-configures serialport interface.

use std::{borrow::Cow, time::Duration};

/// Get a new serialport interface.
///
/// This function is only available with the "serialport" feature. You can use it to just provide a
/// port and get back a `serialport::SerialPortBuilder` with the proper configuration for the
/// Lakeshore336.
///
/// Note that this will also set the timeout for the instrument to 3 seconds.
pub fn new_serialport<'a>(port: impl Into<Cow<'a, str>>) -> serialport::SerialPortBuilder {
    serialport::new(port, 57600)
        .timeout(Duration::from_secs(3))
        .parity(serialport::Parity::Odd)
        .data_bits(serialport::DataBits::Seven)
        .stop_bits(serialport::StopBits::One)
}
