use std::time::Duration;

#[derive(Debug)]
pub struct State {
    pub joystick_x : i8,
    pub joystick_y : i8,
    pub rot_z : i8,
    pub switch : u8
}

impl TryFrom<&[u8]> for State {
    type Error = syiot::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() == 4 {
            unsafe {
                Ok(std::mem::transmute_copy::<[u8; 4], _>(&value.try_into()?))
            }
        } else {
            Err(format!("Wrong size! {}", value.len()).into())
        }
    }
}

fn main() -> Result<(), syiot::Error> {
    let port_name = "COM3";
    let baud_rate = 115200;
    let timeout = Duration::from_millis(1000);

    let mut tele = syiot::tele::SerialPortTele::open(port_name, baud_rate, timeout)?;

    loop {
        let test : State = tele.request()?;
        dbg!(test);
    }
}