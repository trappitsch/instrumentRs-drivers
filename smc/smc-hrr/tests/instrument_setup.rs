//! Test general instrument setup.

use instrumentrs::{smock, u};

use measurements::Pressure;

use proptest::prelude::*;

mod helpers;
use helpers::extend_with_crc;
use smc_hrr::{InstrumentParameter, SmcHrr, types::SlaveAddress};

proptest! {
    #[test]
    fn get_discharge_pressure_with_various_base_addresses(addr in 1_u8..=32) {

        let mut exp_writes = [vec![addr, 0x04, 0x00, 0x02, 0x00, 0x01]];
        extend_with_crc(&mut exp_writes);

        let mut exp_reads = [
            vec![0x01, 0x04, 0x02, 0x00, 0xAA], // answer chiller is on
        ];
        extend_with_crc(&mut exp_reads);

        let mut inst = smock!(SmcHrr, exp_reads, exp_writes);

        let slave_address = SlaveAddress::try_from_writable(format!("{addr:02}")).unwrap();
        inst.set_slave_address(slave_address);

        // ensure command sending is done with correct slave address
        let val: Pressure = u!(inst.get_discharge_pressure()).into();
        assert_eq!(val.as_kilopascals(), 170.0);

        // ensure retrieving slave address works
        assert_eq!(inst.get_slave_address(), slave_address);
    }
}
