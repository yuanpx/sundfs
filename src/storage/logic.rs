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
use storage::nio::NetService;

pub struct Service {
    cmd_channel: Option<UnboundedSender<NetCommand>>,
}
pub trait Process {
    fn process_init(&mut self, service: &mut Service); 
    fn process_event(&mut self, service: &mut Service, ev: NetEvent);
}

impl Service {
    pub fn new() -> Service {
        Service{
            cmd_channel: None,
        }
    }

    pub fn listen(&mut self, id: usize, address: SocketAddr) {
        self.cmd_channel.as_mut().unwrap().send(NetCommand::LISTEN((id,address))).unwrap();
    }

    pub fn connect(&mut self, id: usize, address: SocketAddr) {
        self.cmd_channel.as_mut().unwrap().send(NetCommand::CONNECT((id,address))).unwrap();
    }

    pub fn send(&mut self, id: usize, buf: Vec<u8>) {
        self.cmd_channel.as_mut().unwrap().send(NetCommand::SEND((id, buf))).unwrap();
    }

    pub fn timeout(&mut self, id: usize, sec: usize) {
        self.cmd_channel.as_mut().unwrap().send(NetCommand::TIMEOUT((id, sec))).unwrap();
    }

    pub fn cancel_timeout(&mut self, id: usize) {
        self.cmd_channel.as_mut().unwrap().send(NetCommand::CANCEL_TIMEOUT(id)).unwrap();
    }

    pub fn start<T: Process>(&mut self, process: &mut T) -> Result<()> {
        let mut core = Core::new().unwrap();
        let mut net_service = NetService::new().unwrap();
        let (cmd_tx, event_rx) = net_service.start().unwrap();
        self.cmd_channel = Some(cmd_tx);
        process.process_init(self);
        let event_process = event_rx.fold((self, process), move |(service,process), event|{
            process.process_event(service, event);
            Ok((service, process))
        }).map(|_|());

        core.run(event_process).unwrap(); 
        Ok(())
    }
}
