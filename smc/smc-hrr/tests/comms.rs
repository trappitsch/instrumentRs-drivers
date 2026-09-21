//! Tests for communications.

use std::time::{Duration, Instant};

use instrumentrs::{smock, u};

mod helpers;
use helpers::extend_with_crc;
use smc_hrr::{SmcHrr, types::OperationStatus};

#[test]
fn test_that_silent_interval_between_writes_is_long_enough() {
    let mut exp_writes = [
        vec![0x01, 0x06, 0x00, 0x0C, 0x00, 0x01], // turn chiller on
        vec![0x01, 0x06, 0x00, 0x0C, 0x00, 0x01], // turn chiller on
    ];
    extend_with_crc(&mut exp_writes);

    let mut exp_reads = [
        vec![0x01, 0x06, 0x00, 0x0C, 0x00, 0x01], // answer to turning chiller on
        vec![0x01, 0x06, 0x00, 0x0C, 0x00, 0x01], // answer to turning chiller on
    ];
    extend_with_crc(&mut exp_reads);

    let mut inst = smock!(SmcHrr, exp_reads, exp_writes);

    let tic = Instant::now();
    u!(inst.set_operation_status(OperationStatus::Run));
    let toc = Instant::now();
    u!(inst.set_operation_status(OperationStatus::Run));

    // 2 times 3.5 characters at baud 9600 is 8.021 ms.
    assert!(toc - tic > Duration::from_micros(6563));
}
