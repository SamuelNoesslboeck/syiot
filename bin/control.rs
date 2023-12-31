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
        } else {
            dbg!(msg.err());
        }
    }
}

fn main() -> Result<(), syiot::Error> {
    let remote = Control::new(Handler { }); 

    remote.listen(Transport::Tcp, "0.0.0.0:12200")?;
    remote.run();

    Ok(())
}