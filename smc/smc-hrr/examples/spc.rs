//! Tests for the SmcHrr driver.

use std::{net::TcpStream, time::Duration};

use smc_hrr::{SmcHrr, types::SerialRemoteInstruction};

fn main() {
    // let stream = TcpStream::connect(sock_addr)?;
    // let timeout = Duration::from_secs(3);
    // stream.set_write_timeout(Some(timeout))?;
    // stream.set_read_timeout(Some(timeout))?;
    let interface = TcpStream::connect("192.168.127.254:4002").unwrap();
    interface
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();
    interface
        .set_write_timeout(Some(Duration::from_secs(1)))
        .unwrap();

    let mut chiller = SmcHrr::new(interface);

    // Set chiller to serial
    chiller
        .set_serial_remote_instructions(SerialRemoteInstruction::Serial)
        .unwrap();
    // chiller
    // .set_serial_remote_instructions(SerialRemoteInstruction::LocalDio)
    // .unwrap();
    // let new_st = Temperature::from_celsius(20.);
    // chiller.set_set_temperature(new_st.into()).unwrap();
    //
    // println!(
    //     "Set temperature: {}",
    //     chiller.get_set_temperature().unwrap()
    // );

    let current_op_status = chiller.get_operation_status().unwrap();
    println!("Operation status: {:?}", current_op_status);

    println!(
        "Set temperature: {}",
        chiller.get_set_temperature().unwrap()
    );
    println!(
        "Discharge Temperature: {}",
        chiller.get_discharge_temperature().unwrap()
    );
    println!("Flow rate: {}", chiller.get_flow_rate().unwrap());
    println!(
        "Discharge pressure: {}",
        chiller.get_discharge_pressure().unwrap()
    );
    println!(
        "ElectricConductivity: {}",
        chiller.get_electric_conductivity().unwrap()
    );

    println!("Status: {:?}", chiller.get_status().unwrap());
    println!("Alarms: {}", chiller.get_alarm_flags().unwrap());

    std::thread::sleep(Duration::from_secs(3));

    // chiller
    //     .set_serial_remote_instructions(SerialRemoteInstruction::LocalDio)
    //     .unwrap();
}
