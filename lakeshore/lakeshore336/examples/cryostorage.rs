//! Hook up to cryostorage via USB/serialport.

use lakeshore336::{Input, Lakeshore336, new_serialport};
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
