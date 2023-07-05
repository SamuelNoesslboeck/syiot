extern crate alloc;

use core::mem::size_of;
use core::time::Duration;
use std::{ffi::OsStr, io::{Read, Write}, thread::JoinHandle, sync::Mutex, os::windows::raw::SOCKET};

use alloc::sync::Arc;
use serial::SerialPort;
#[cfg(windows)]
use serial::windows::COMPort;

pub const SEP_CHAR : u8 = ' ' as u8;

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
            let mut buf_len = 1;
            buf[0] = SEP_CHAR;

            loop {
                let mut port = port_mut.lock().unwrap();
                match port.read(&mut buf[buf_len..]) {
                    Ok(len) => {
                        buf_len += len;
                    },
                    Err(err) => {
                        if !err.to_string().contains("timed out") {
                            panic!("Thread paniced! {}", err);
                        }
                    }
                };
                drop(port);

                let mut start_index = None;
                let mut stop_index = None;

                for i in 0 .. buf.len() {
                    if buf[i] == SEP_CHAR {
                        if start_index.is_some() {
                            stop_index = Some(i);
                            break;
                        } else {
                            start_index = Some(i); 
                        }
                    }
                }
                
                if let Some((start, stop)) = start_index.zip(stop_index) {
                    let mut state = state_mut.lock().unwrap();



                    drop(state);
                }
            }
        }));
    }
}
