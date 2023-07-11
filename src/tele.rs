use core::marker::PhantomData;
use core::mem::size_of;
use std::time::Duration;

use serialport::SerialPort;

const REQUEST_BUF : [u8; 1] = [ 1 ];

pub struct SerialPortTele<T : for<'b> TryFrom<&'b [u8]>> {
    port : Box<dyn SerialPort>,
    buf : Vec<u8>,

    pdata : PhantomData<T>
}

impl<T : for<'b> TryFrom<&'b [u8], Error = crate::Error>> SerialPortTele<T> {
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

    pub fn request(&mut self) -> Result<T, crate::Error> {
        self.port.write(&REQUEST_BUF)?;     // Request data

        let read_len = self.port.read(self.buf.as_mut_slice())?;
        T::try_from(&self.buf[..read_len])
    }

    pub fn request_checked(&mut self) -> Result<T, crate::Error> {
        self.port.write(&REQUEST_BUF)?;     // Request data

        let mut read_len : usize;
        loop {
            read_len = self.port.read(self.buf.as_mut_slice())?;

            if read_len == size_of::<T>() {
                break;
            } else {
                dbg!(read_len);
            }
        }

        T::try_from(&self.buf[..read_len])
    }
}