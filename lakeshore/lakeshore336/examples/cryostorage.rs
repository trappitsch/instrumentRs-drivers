//! Hook up to cryostorage via USB/serialport.

use std::{thread, time::Duration};

use lakeshore336::{
    Input, Lakeshore336, Output, new_serialport,
    types::{
        HeaterMaxOutputCurrent, HeaterOutputDisplay, HeaterResistance, HeaterSetup, OnPowerup,
        OutputMode, OutputModeSetup, heater_setup::HeaterRange,
    },
};
use measurements::Temperature;
use serialport::SerialPortType;

fn main() {
    let product_desc = "Model 336 Temperature Controller";
    let port = find_port_by_product(product_desc);
    let interface = new_serialport(port).open().unwrap();

    let mut inst = Lakeshore336::new(interface);

    println!("Name: {}", inst.get_name().unwrap());
    println!(
        "Ch A: {}",
        inst.channel(Input::InA).get_temperature().unwrap()
    );
    println!(
        "Ch B: {}",
        inst.channel(Input::InB).get_temperature().unwrap()
    );

    println!("{}", inst.channel(Input::InA).get_channel_name().unwrap());
    println!("{}", inst.channel(Input::InB).get_channel_name().unwrap());

    inst.channel(Input::InA).set_channel_name("Sample").unwrap();

    // HEATER SETUP - ALL TESTED AND WORKING
    //
    // let heater_setup = HeaterSetup::try_new(
    //     HeaterResistance::R25Ohm,
    //     HeaterMaxOutputCurrent::C2A,
    //     HeaterOutputDisplay::Current,
    // )
    // .unwrap();
    // let out = Output::Out1;
    // inst.output(out).set_heater_setup(heater_setup).unwrap();
    //
    // println!(
    //     "Heater setup: {:?}",
    //     inst.output(out).get_heater_setup().unwrap()
    // );
    //
    // let output_mode_setup = OutputModeSetup::new(
    //     OutputMode::ClosedLoopPID,
    //     Some(Input::InA),
    //     OnPowerup::Disabled,
    // );
    // inst.output(out).set_output_mode(output_mode_setup).unwrap();
    //
    // println!(
    //     "Output mode: {:?}",
    //     inst.output(out).get_output_mode().unwrap()
    // );
    //
    // inst.output(out)
    //     .set_setpoint(Temperature::from_kelvin(120.))
    //     .unwrap();
    //
    // println!("Setpoint: {:?}", inst.output(out).get_setpoint().unwrap());
    //
    // inst.output(out).set_range(HeaterRange::Medium).unwrap();
    // println!("Heater Range: {:?}", inst.output(out).get_range().unwrap());
    //
    // for _ in 0..30 {
    //     println!(
    //         "Heater power: {}%",
    //         inst.output(out).get_heater_output().unwrap().as_percent()
    //     );
    //     thread::sleep(Duration::from_secs(1));
    // }
}

/// Find the port the Lakeshore is connected to (or panic).
pub fn find_port_by_product(needle: &str) -> String {
    let ports = serialport::available_ports().unwrap();

    let port_name = ports.iter().find_map(|info| {
        if let SerialPortType::UsbPort(usb) = &info.port_type {
            // `product` is an `Option<String>`.
            usb.product
                .as_ref()
                .filter(|prod| prod.contains(needle))
                .map(|_| info.port_name.clone())
        } else {
            None
        }
    });

    port_name.unwrap()
}
