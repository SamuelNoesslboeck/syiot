use core::mem::size_of;
use std::io::{self, Write};
use std::time::Duration;

#[derive(Debug)]
struct TestStruct {
    test : u8,
    int_test : i8
}

fn main() {
    let port_name = "COM3";
    let baud_rate = 115200;

    let port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(10))
        .data_bits(serialport::DataBits::Eight)
        .open();

    match port {
        Ok(mut port) => {
            let mut serial_buf: Vec<u8> = vec![0; 1000];
            println!("Receiving data on {} at {} baud:", &port_name, &baud_rate);
            loop {
                match port.read(serial_buf.as_mut_slice()) {
                    Ok(t) => { 
                        // io::stdout().write_all(&serial_buf[..t]).unwrap();

                        if t == size_of::<TestStruct>() {
                            unsafe {
                                let test : TestStruct = std::mem::transmute_copy::<[u8; size_of::<TestStruct>()], _>(&serial_buf[..size_of::<TestStruct>()].try_into().unwrap());
                                dbg!(test);
                            }
                        } else {
                            println!("Bad size: {}", t);
                        }
                    },
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
            ::std::process::exit(1);
        }
    }
}