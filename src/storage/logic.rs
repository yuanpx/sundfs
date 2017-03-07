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
use self::futures::sync::mpsc::UnboundedReceiver;
use self::futures::sync::mpsc;
use storage::nio::NetCommand;
use storage::nio::NetEvent;
use storage;

pub struct Service {
    cmd_channel: UnboundedSender<NetCommand>,
    event_channel: UnboundedReceiver<NetEvent>,
}
pub trait Process {
    fn process_event(&mut self, service: &mut Service, ev: NetEvent)
    {}
}

impl Service {
    pub fn new(cmd_channel: UnboundedSender<NetCommand>, event_channel: UnboundedReceiver<NetEvent>) -> Service {
        Service{
            cmd_channel: cmd_channel,
            event_channel: event_channel,
        }
    }

    pub fn listen(&mut self, address: SocketAddr) {
        self.cmd_channel.send(NetCommand::LISTEN(address)).unwrap();
    }

    pub fn connect(&mut self, address: SocketAddr) {
        self.cmd_channel.send(NetCommand::CONNECT(address)).unwrap();
    }

    pub fn send(&mut self, id: usize, buf: Vec<u8>) {
        self.cmd_channel.send(NetCommand::SEND((id, buf))).unwrap();
    }

    pub fn start<T: Process>(&mut self, process: &mut T) -> Result<()> {
        let mut core = Core::new().unwrap();
        let (tx, rx) = mpsc::unbounded();
        let event_process = rx.fold((self, process), move |(service,process), event|{
            process.process_event(service, event);
            Ok((service, process))
        }).map(|_|());

        core.run(event_process).unwrap(); 
        Ok(())
    }
}
