//! Types that are used for communication settings.

use std::time::{Duration, Instant};

use crate::InstrumentParameter;

/// Define the total minimum silent interval between writes (see `SilentInterval`)
static SILENT_INT: Duration = Duration::from_micros(6563);

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

/// Manage silent interval in communications.
///
/// MODBUS requires us that we have a silent interval at the beginning and at the end of each
/// transission of at least 3.5 characters. This helper facilitates complying with this rule.
/// As the chiller can run at two baud rates but we don't know which one they chose, we will use the
/// slower one here to enforce the silent interval. The silent interval is still only 8.021 ms for 7
/// characters (3.5 at start and 3.5 at end).
///
/// The silent interval is define in the static `SILENT_INT` in this module.
pub struct SilentInterval {
    last_write: Instant,
}

impl SilentInterval {
    /// Block thread (sleep) until silent interval has expired.
    ///
    /// This checks how much time expired since the last read. If this is smaller than the required
    /// silent interval, it will block the thread for the time required, afterwards update the last
    /// write time and continue.
    pub fn block(&mut self) {
        let dt = Instant::now() - self.last_write;

        if dt < SILENT_INT {
            std::thread::sleep(SILENT_INT - dt);
        }

        self.last_write = Instant::now();
    }
}

impl Default for SilentInterval {
    fn default() -> Self {
        Self {
            last_write: Instant::now(),
        }
    }
}
