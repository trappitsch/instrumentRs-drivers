//! Tests the instrument queries for Lakeshore336.

use instrumentrs2::{smock, u};

use lakeshore336::Lakeshore336;

pub static TERM: &str = "\n";

#[test]
fn get_name() {
    let expected_writes = ["*IDN?"];
    let expected_reads = ["Lakeshore,336,123456/42,42.13"];

    let mut inst = smock!(Lakeshore336, expected_reads, expected_writes, TERM);

    let name = u!(inst.get_name());
    assert_eq!(name, expected_reads[0]);
}
