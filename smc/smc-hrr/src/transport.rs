use std::io::{Read, Write};

use instrumentrs::{
    InstrumentError,
    transport::{Transport, fn_sync::read_number_of_bytes},
};

use crate::{
    InstrumentParameter, SmcHrr,
    types::{SilentInterval, SlaveAddress},
};

impl<I> Transport<&[u8], Vec<u8>> for SmcHrr<I>
where
    I: Read + Write,
{
    type Channel = u8;

    fn sendcmd(
        &mut self,
        cmd: &[u8],
        _idx: Option<Self::Channel>,
        args: Option<&[&[u8]]>,
    ) -> Result<(), instrumentrs::InstrumentError> {
        let pkg = make_package(0x06, cmd, &self.slave_address, args);

        let fn_code = pkg[1];

        write_package(&mut self.interface, &mut self.silent_interval, &pkg)?;

        let mut answer_pkg = read_number_of_bytes(&mut self.interface, 2)?;

        if fn_code == answer_pkg[1] {
            answer_pkg
                .extend_from_slice(&read_number_of_bytes(&mut self.interface, pkg.len() - 2)?);

            check_crc(&answer_pkg)
        } else {
            // negative answer, we expect three more bytes: error code and CRC
            answer_pkg.extend_from_slice(&read_number_of_bytes(&mut self.interface, 3)?);
            Err(process_negative_answer(&answer_pkg))
        }
    }

    fn query(
        &mut self,
        cmd: &[u8],
        _idx: Option<Self::Channel>,
        args: Option<&[&[u8]]>,
    ) -> Result<Vec<u8>, instrumentrs::InstrumentError> {
        let pkg = make_package(0x04, cmd, &self.slave_address, args);

        let fn_code = pkg[1];

        write_package(&mut self.interface, &mut self.silent_interval, &pkg)?;

        let mut answer_pkg = read_number_of_bytes(&mut self.interface, 3)?;

        if fn_code == answer_pkg[1] {
            answer_pkg.extend_from_slice(&read_number_of_bytes(
                &mut self.interface,
                answer_pkg[2] as usize,
            )?);
            answer_pkg.extend_from_slice(&read_number_of_bytes(&mut self.interface, 2)?);

            check_crc(&answer_pkg)?;

            Ok(answer_pkg[3..answer_pkg.len() - 2].into())
        } else {
            // negative answer, we expect two more bytes (CRC)
            answer_pkg.extend_from_slice(&read_number_of_bytes(&mut self.interface, 2)?);
            Err(process_negative_answer(&answer_pkg))
        }
    }
}

/// Make a package.
fn make_package(
    fn_code: u8,
    cmd: &[u8],
    slave_address: &SlaveAddress,
    args: Option<&[&[u8]]>,
) -> Vec<u8> {
    // stitch together address, cmd, and data.
    let mut pkg: Vec<u8> = slave_address.to_writable();
    pkg.push(fn_code);
    pkg.extend_from_slice(cmd);
    if let Some(dd) = args {
        for d in dd {
            pkg.extend_from_slice(d);
        }
    }

    // crc
    let crc = calculate_crc(&pkg);

    pkg.extend_from_slice(&crc);

    pkg
}

fn calculate_crc(data: &[u8]) -> [u8; 2] {
    let mut result = u16::MAX;

    for b in data {
        result ^= *b as u16;

        for _ in 0..8 {
            let ex_or_a001 = result & 1 == 1;
            result >>= 1;

            if ex_or_a001 {
                result ^= 0xa001;
            }
        }
    }

    result.to_le_bytes()
}

/// Check the CRC of a given package.
///
/// Takes the package minus the last two, calculates the CRC, and then compares the CRC with the
/// last two. If good, an `Ok(())` is returned, otherwise an `InstrumentError::CheckusmError`.
///
/// We always must provide a package to this function that contains at least 2 entries!
fn check_crc(pkg: &[u8]) -> Result<(), InstrumentError> {
    let crc_exp = calculate_crc(&pkg[..pkg.len() - 2]);

    if crc_exp == pkg[pkg.len() - 2..] {
        Ok(())
    } else {
        Err(InstrumentError::ChecksumError)
    }
}

/// Process a negative answer.
///
/// If a negative answer was received, function processes it and then errors.
/// Possible errors are (in order or priority):
/// - `InstrumentError::NegativeResponse`:
///   - Unspecified function code used.
///   - Address is out of range.
///   - Data field is not normal.
/// - `InstrumentError::CheckusmError`:
///   - CRC does not match.
/// - `InstrumentError::NegativeResponse` unknown error occured. This should never happen and if
///   it does, most likely the interface needs to flushed now.
///
///   The package that is provided must have be 5 bytes long.
fn process_negative_answer(pkg: &[u8]) -> InstrumentError {
    match pkg[2] {
        0x01 => InstrumentError::NegativeResponse {
            msg: "function code unknown".to_string(),
        },
        0x02 => InstrumentError::NegativeResponse {
            msg: "address is out of range".to_string(),
        },
        0x03 => InstrumentError::NegativeResponse {
            msg: "data field is not normal".to_string(),
        },
        _ => {
            if let Err(e) = check_crc(pkg) {
                e
            } else {
                InstrumentError::NegativeResponse {
                    msg: "unknown error: interface likely needs to be flushed".to_string(),
                }
            }
        }
    }
}

fn write_package<I: Write>(
    interface: &mut I,
    silent_int: &mut SilentInterval,
    pkg: &[u8],
) -> Result<(), InstrumentError> {
    silent_int.block();

    interface.write_all(pkg)?;
    interface.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_crc_in_calculation_example_page_3_11() {
        let data = vec![0x01, 0x06, 0x00, 0x0B, 0x00, 0xFE];
        let crc = calculate_crc(&data);
        assert_eq!(crc, [0x79, 0x88]);
    }
}
