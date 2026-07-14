//! Implements instrumentRs transport trait for Lakeshore336.

use std::io::{Read, Write};

use instrumentrs2::transport::{Transport, Writable, read_until_terminator, write_all};

use crate::{InstrumentError, Lakeshore336, Parameter, channel::Channel};

impl<I> Transport<&str, String> for Lakeshore336<I>
where
    I: Read + Write,
{
    type Channel = Channel;
    fn sendcmd(
        &mut self,
        cmd: &str,
        idx: Option<Channel>,
        args: Option<&[&str]>,
    ) -> Result<(), InstrumentError> {
        let pkg = make_package(cmd, idx, args);
        write_all(
            &mut self.interface,
            pkg.to_byte_slice(),
            self.terminator.to_byte_slice(),
        )
    }

    fn query(
        &mut self,
        cmd: &str,
        idx: Option<Channel>,
        args: Option<&[&str]>,
    ) -> Result<String, InstrumentError> {
        self.sendcmd(cmd, idx, args)?;
        let res = read_until_terminator(&mut self.interface, self.terminator.to_byte_slice())?;
        Ok(String::from_utf8(res)?)
    }
}

/// Makes a package for sendcommand.
fn make_package(cmd: &str, idx: Option<Channel>, args: Option<&[&str]>) -> String {
    match (idx, args) {
        // No channel, no arguments
        (None, None) => String::from(cmd),
        // No channel, arguments
        (None, Some(args)) => {
            let mut cmd_str = format!("{} ", cmd);
            for arg in args {
                cmd_str.push(',');
                cmd_str.push_str(arg);
            }
            cmd_str
        }
        // Channel, no arguments
        (Some(idx), None) => {
            format!("{} {}", cmd, idx.to_writable())
        }
        // Channel and arguments
        (Some(idx), Some(args)) => {
            let mut cmd_str = format!("{} {}", cmd, idx.to_writable());
            for arg in args {
                cmd_str.push(',');
                cmd_str.push_str(arg);
            }
            cmd_str
        }
    }
}
