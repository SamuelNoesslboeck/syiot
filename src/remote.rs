use core::mem::size_of;
use core::marker::PhantomData;
use std::net::ToSocketAddrs;

use message_io::node::{self, NodeHandler, NodeListener};
use message_io::network::{NetEvent, Endpoint, ToRemoteAddr};

use crate::{TryFromBytes, IntoBytes};

// Public imports
pub use message_io::network::Transport;

/// Handler trait with event functions for the [Control] struct
pub trait ControlHandler<M : TryFromBytes> {
    /// Function that will be called once the `Control` has accept a client
    fn on_accept(&mut self); 

    /// Function that will be called once the `Control` received a message
    fn on_msg(&mut self, msg : Result<M, crate::Error>);
}

/// 
pub struct Control<T : TryFromBytes, H : ControlHandler<T>> {
    handler : H,
    net_handler : NodeHandler<()>,
    net_listener : NodeListener<()>,
    _data : PhantomData<T>
}

impl<T : TryFromBytes, H : ControlHandler<T>> Control<T, H> {
    /// Create a new `Control` using the given handler
    pub fn new(handler :  H) -> Self {
        let (net_handler, net_listener) = node::split::<()>();

        Self {
            handler, 
            net_handler,
            net_listener,
            _data: PhantomData::default()
        }
    }

    /// Listen at a specific address 
    pub fn listen(&self, transport : Transport, addr : impl ToSocketAddrs) -> Result<(), crate::Error> {
        self.net_handler.network().listen(transport, addr)?;
        Ok(())
    }

    /// Starts the control and the message handling, blocks the current thread!
    pub fn run(mut self) {
        self.net_listener.for_each(move |event| match event.network() {
            NetEvent::Accepted(_ep, _r_id) => self.handler.on_accept(),
            NetEvent::Message(_ep, msg) => self.handler.on_msg(T::try_from(msg)),
            _ => { }
        })
    }
}

/// Network client for the `Control` struct
pub struct ControlClient<T> {
    server : Option<Endpoint>,

    net_handler : NodeHandler<()>,
    // net_listener : NodeListener<()>,

    _data : PhantomData<T>
}

impl<T> ControlClient<T> {
    /// Creates a new client
    pub fn new() -> Self {
        let ( net_handler, _net_listener ) = node::split::<()>();

        Self {
            server: None,
            net_handler, 
            _data: PhantomData::default()
        }
    }

    /// Connect to a `Control` server
    pub fn connect(&mut self, transport : Transport, addr : impl ToRemoteAddr) -> Result<(), crate::Error> {
        self.server = Some(self.net_handler.network().connect(transport, addr)?.0);
        Ok(())
    }

    /// Send an instance of the type to the server
    pub fn send(&self, data : &T) -> Result<(), crate::Error> {
        if let Some(server) = self.server {
            let bytes = unsafe { core::slice::from_raw_parts((data as *const T) as *const u8, size_of::<T>()) };
            self.net_handler.network().send(server, bytes); 
            Ok(())
        } else {
            Err("You must connect to a server first!".into())
        }
    }
}

impl<T : IntoBytes> ControlClient<T> {
    /// Sends an instance of the given value to the server (copies it in the process)
    pub fn send_copy(&self, data : T) -> Result<(), crate::Error> {
        if let Some(server) = self.server {
            let data : Vec<u8> = data.into();
            self.net_handler.network().send(server, &data); 
            Ok(())
        } else {
            Err("You must connect to a server first!".into())
        }
    }
}