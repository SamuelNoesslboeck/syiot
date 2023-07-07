use std::time::Duration;

#[derive(Debug)]
struct TestStruct {
    test : u8,
    int_test : i8
}

impl TryFrom<&[u8]> for TestStruct {
    type Error = syiot::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() == 2 {
            unsafe {
                Ok(std::mem::transmute_copy::<[u8; 2], _>(&value.try_into()?))
            }
        } else {
            Err("Wrong size!".into())
        }
    }
}

fn main() -> Result<(), syiot::Error> {
    let port_name = "COM3";
    let baud_rate = 115200;
    let timeout = Duration::from_millis(1000);

    let mut tele = syiot::tele::SerialPortTele::open(port_name, baud_rate, timeout)?;

    loop {
        let test : TestStruct = tele.recv()?;
        dbg!(test);
    }
}