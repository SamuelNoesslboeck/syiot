use core::marker::PhantomData;
use core::mem::size_of;
use std::time::Duration;

use serialport::SerialPort;

const REQUEST_BUF : [u8; 1] = [ 1 ];

use crate::TryFromBytes;

/// A struct for using telemetry using a serial port 
pub struct SerialPortTele<T : TryFromBytes> {
    port : Box<dyn SerialPort>,
    buf : Vec<u8>,

    pdata : PhantomData<T>
}

impl<T : TryFromBytes<Error = crate::Error>> SerialPortTele<T> {
    /// Open the serial port to listen for telemetry data
    pub fn open<'a>(port : impl Into<std::borrow::Cow<'a, str>>, baud_rate : u32, timeout : Duration) -> Result<Self, crate::Error> {
        Ok(Self {
            port: serialport::new(port, baud_rate)
                .timeout(timeout)
                .data_bits(serialport::DataBits::Eight)
                .open()?,
            buf: vec![0; size_of::<T>()],
            pdata: PhantomData::default()
        })
    }

    /// Request data from the serial port 
    pub fn request(&mut self) -> Result<T, crate::Error> {
        self.port.write(&REQUEST_BUF)?;     // Request data

        let read_len = self.port.read(self.buf.as_mut_slice())?;
        T::try_from(&self.buf[..read_len])
    }

    /// Request data from the serial port, checks if the given data has the size of T, will not parse the value otherwise
    pub fn request_checked(&mut self) -> Result<T, crate::Error> {
        self.port.write(&REQUEST_BUF)?;     // Request data

        let mut read_len : usize;
        loop {
            read_len = self.port.read(self.buf.as_mut_slice())?;

            if read_len == size_of::<T>() {
                break;
            } 
        }

        T::try_from(&self.buf[..read_len])
    }
}