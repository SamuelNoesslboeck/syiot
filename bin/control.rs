use core::mem::size_of;

use syiot::remote::{Control, ControlHandler, Transport};

#[allow(unused)]
#[derive(Debug, Clone)]
enum Command {
    Test { val : u8 }, 
    Run { }
}

syiot::impl_tryfrom_transmute!(Command);

pub struct Handler { }

impl ControlHandler<Command> for Handler {
    fn on_accept(&mut self) {
        println!(" => Accepted!");
    }

    fn on_msg(&mut self, msg : Result<Command, syiot::Error>) {
        if let Ok(cmd) = msg {
            dbg!(cmd);
        }
    }
}

fn main() -> Result<(), syiot::Error> {
    let remote = Control::new(Handler { }); 

    remote.listen(Transport::FramedTcp, "127.0.0.1:12200")?;
    remote.run();

    Ok(())
}