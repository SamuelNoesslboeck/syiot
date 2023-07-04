extern crate alloc;

use core::mem::size_of;
use core::time::Duration;
use std::{ffi::OsStr, io::{Read, Write}, thread::JoinHandle, sync::Mutex};

use alloc::sync::Arc;
use serial::SerialPort;
#[cfg(windows)]
use serial::windows::COMPort;

pub const SEP_CHAR : char = ' ';

pub struct Base64Serial<T : TryFrom<Vec<u8>> + Default + Send + 'static> {
    #[cfg(windows)]
    port : Arc<Mutex<COMPort>>,
    state : Arc<Mutex<T>>,

    thr : Option<JoinHandle<()>>
}

impl<T : TryFrom<Vec<u8>> + Default + Send + 'static> Base64Serial<T> {
    pub fn connect<S : AsRef<OsStr> + ?Sized>(interface : &S) -> Result<Self, crate::Error> {
        let mut port = serial::open(interface)?;
        port.reconfigure(&|settings| {
            settings.set_baud_rate(serial::Baud115200)?;
            settings.set_char_size(serial::Bits8);
            settings.set_parity(serial::ParityNone);
            settings.set_stop_bits(serial::Stop1);
            settings.set_flow_control(serial::FlowNone);
            Ok(())
        })?;

        port.set_timeout(Duration::from_millis(50))?;
    
        Ok(Self { 
            port: Arc::new(Mutex::new(port)), 
            state: Default::default(),
            thr: None 
        })
    }

    pub fn listen(&mut self) {
        let port_mut = self.port.clone();
        let state_mut = self.state.clone();

        self.thr = Some(std::thread::spawn(move || {
            let mut buf = Vec::with_capacity(size_of::<T>()*2);

            loop {
                let mut port = port_mut.lock().unwrap();
                if let Err(err) = port.read_exact(&mut buf) {
                    if !err.to_string().contains("timed out") {
                        panic!("Thread paniced! {}", err);
                    }
                };
                drop(port);

                let mut start_index = None;
                let mut stop_index = None;

                for i in 0 .. buf.len() {

                }

                let mut state = state_mut.lock().unwrap();
        


                drop(state);
            }
        }));
    }
}
