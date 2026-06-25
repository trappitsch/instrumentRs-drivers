//! Tests for the temperature channels of the Lakeshore336.

use instrumentrs2::{smock, u};

use lakeshore336::{Input, Lakeshore336};
use measurements::test_utils::assert_almost_eq;

pub static TERM: &str = "\n";

#[test]
fn get_temperature() {
    let expected_writes = ["KRDG? A", "KRDG? C"];
    let expected_reads = ["273.15", "42"];

    let mut inst = smock!(Lakeshore336, expected_reads, expected_writes, TERM);

    // check channel A
    let mut ch_a = inst.channel(Input::InA);
    let temp_a = u!(ch_a.get_temperature());
    assert_almost_eq(temp_a.as_kelvin(), 273.15);

    let mut ch_c = inst.channel(Input::InC);
    let temp_c = u!(ch_c.get_temperature());
    assert_almost_eq(temp_c.as_kelvin(), 42.);
}

#[test]
fn sensor_name_set() {
    let expected_writes = [
        "INNAME A,\"Suzan Kelvin\"",
        "INNAME C,\"Karl234\"",
        "INNAME? B",
    ];
    let expected_reads = ["\"Even colder\""];

    let mut inst = smock!(Lakeshore336, expected_reads, expected_writes, TERM);

    u!(inst.channel(Input::InA).set_channel_name("Suzan Kelvin"));
    u!(inst.channel(Input::InC).set_channel_name("Karl234"));

    let name_b = u!(inst.channel(Input::InB).get_channel_name());
    assert_eq!(name_b, "Even colder");
}
