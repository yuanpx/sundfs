extern crate futures;
use std::collections::HashMap;
use std::rc::Rc;
use std::iter;
use std::env;
use std::io::Result;
use std::net::SocketAddr;
use std::io::{Error, ErrorKind};
use std::time::Duration;
use std::vec::Vec;
use std::cell::RefCell;
use std::cell::Cell;
extern crate tokio_core;
use self::tokio_core::net::TcpListener;
use self::tokio_core::net::TcpStream;
use self::tokio_core::reactor::Core;
use self::tokio_core::reactor::Timeout;
use self::tokio_core::reactor::Interval;
use self::tokio_core::reactor::Handle;
use self::tokio_core::io;
use self::tokio_core::io::Io;
use self::futures::stream;
use self::futures::stream::Stream;
use self::futures::Future;
use self::futures::sync::mpsc::UnboundedSender;
use self::futures::sync::mpsc;

pub trait Service {
    fn process_event(ev: NetEvent);
}

pub struct EventLoop<T> {
    pub net: NetService,
    pub core: Core,
    service: Option<T>;
}



impl <T: Service>EventLoop<T> {
    pub fn new(t: T) -> EventLoop {
        net: NetService::new().unwrap(),
        messages: None,
        service: Some(t),
    }

    pub fn listen(&mut self, address: SocketAddr) {
        self.net.listen(address);
    }

    pub fn connect(&mut self, address: SocketAddr) {
        self.net.connect(address);
    }

    pub fn start(&mut self) -> Result<()> {
        let handle = self.core.hadnle();
        let (tx, rx) = mpsc::unbounded();
        let event_process = rx.fold(, move |count, (id, buf)|{
            let handle_inner = handle_out.clone();
            let id_inner = id_source.clone();
            let connections_inner = connections.clone();
            self::process_command(handle_inner, id_inner, connections_inner, command);
            ok(count + 1)
        }).map(|_|());

        self.commands = some(tx);
        let event_process = rx.fold(0, move |count, command|{
            let handle_inner = handle_out.clone();
            let id_inner = id_source.clone();
            let connections_inner = connections.clone();
            self::process_command(handle_inner, id_inner, connections_inner, command);
            ok(count + 1)
        }).map(|_|());

        try!(self.net.start());
         
    }
}
