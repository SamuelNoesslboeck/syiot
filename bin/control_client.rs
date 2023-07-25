use std::thread::sleep;
use std::time::Duration;

use syiot::remote::{ControlClient, Transport};

#[allow(unused)]
#[derive(Debug, Clone)]
enum Command {
    Test { val : u8 }, 
    Run { }
}

fn main() -> Result<(), syiot::Error> {
    let mut client = ControlClient::<Command>::new();

    client.connect(Transport::Tcp, "127.0.0.1:12200")?;

    loop {
        client.send(&Command::Test { val : 2 })?;
        println!("Sent data!");
        sleep(Duration::from_millis(1000));
    }
}