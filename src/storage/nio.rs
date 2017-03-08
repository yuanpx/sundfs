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
use std::thread;
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

pub enum NetCommand {
    LISTEN((usize,SocketAddr)),
    CONNECT((usize,SocketAddr)),
    SEND((usize, Vec<u8>)),
    TIMEOUT((usize, usize)),
}

pub enum NetEvent {
    CONNECTED((usize, usize)),
    DISCONNECTED((usize, usize)),
    MESSAGE((usize, Vec<u8>)),
    TIMEOUT(usize),
}

pub struct NetService {
}

impl NetService{
    pub fn new() -> Result<NetService> {
        let mut core = try!(Core::new());
        Ok(NetService{
        })
    }

    pub fn process_command(handle: Handle, id_source: Rc<Cell<usize>>,connections: Rc<RefCell<HashMap<usize, UnboundedSender<Vec<u8>>>>>, channel: UnboundedSender<NetEvent>, command: NetCommand) {
        match command {
            NetCommand::LISTEN((id,address)) => {
                Self::process_listen(handle,id, id_source, connections,channel, &address).unwrap();
            },
            NetCommand::CONNECT((id,address)) => {
                Self::process_connect(handle, id, id_source, connections, channel, &address).unwrap();
            },
            NetCommand::SEND((id, buf)) => {
                Self::process_send(id, connections, buf);
                
            },
            NetCommand::TIMEOUT((id, sec)) => {
                Self::process_timeout(handle, id, sec, channel).unwrap();
            },
        }
    }

    pub fn process_stream(event: usize, id: usize, handle: Handle, stream: TcpStream, connections: Rc<RefCell<HashMap<usize, UnboundedSender<Vec<u8>>>>>, channel: UnboundedSender<NetEvent>){
        let handle_inner = handle.clone();
        let channel_out = channel.clone();
        let (tx, rx) = mpsc::unbounded();
        connections.borrow_mut().insert(id, tx);
        let (reader, writer) = stream.split();
        let iter = stream::iter(iter::repeat(()).map(Ok::<(), Error>));
        //let mut header_vec: Vec<u8> = vec![0;4];
        let mut header_vec: Vec<u8> = vec![0;1];
        let socket_reader = iter.fold((reader, channel, id, header_vec), move | (reader,channel,id, header_vec), _| {
            let header = io::read_exact(reader, header_vec);
            let header = header.and_then(move|(reader, header_vec)|{
                if header_vec.len() == 0 {
                    Err(Error::new(ErrorKind::BrokenPipe, "broken pipe"))
                } else {
                    
                    Ok((reader, channel, id,header_vec))
                }
            });
            
            let body = header.and_then(|(reader,channel,id, header_vec)|{
                let body_len = header_vec[0] as usize; 
                let mut body_vec = vec![0; body_len];
                let body_inner = io::read_exact(reader, body_vec);
                let body_inner = body_inner.and_then(move |(reader, body_vec)|{
                    if body_vec.len() == 0 {
                        Err(Error::new(ErrorKind::BrokenPipe, "broken pipe"))
                    } else {
                        Ok((reader, channel,id, header_vec, body_vec))
                    }
                });
                body_inner
            });
            
            //let body = body.map(|(reader, vec)| {
            body.map(|(reader,channel,id, header_vec, body_vec)| {
                // (reader, String::from_utf8(vec))
                println!("{:?}", String::from_utf8(body_vec.clone()));

                channel.send(NetEvent::MESSAGE((id, body_vec))).unwrap();
                (reader, channel,id, header_vec)
            })
        });

        let socket_writer = rx.fold(writer, |writer, msg|{
            let amt = io::write_all(writer, msg);
            let amt = amt.map(|(writer, _)| writer);
            amt.map_err(|_|())
        }).map(|_|());
        
        let socket_reader = socket_reader.map_err(|_| ());
        let connection = socket_reader.map(|_|());
        let connection = connection.select(socket_writer);
        handle_inner.spawn(connection.then(move |_|{
            connections.borrow_mut().remove(&id).unwrap();
            channel_out.send(NetEvent::DISCONNECTED((event,id))).unwrap();
            Ok(())
        }));
    }

    pub fn process_connect(handle: Handle, event: usize, id_source: Rc<Cell<usize>>,connections: Rc<RefCell<HashMap<usize, UnboundedSender<Vec<u8>>>>>, channel: UnboundedSender<NetEvent>, address: &SocketAddr) -> Result<()> {
        let id = id_source.get();
        id_source.set(id + 1);
        let handle_out = handle.clone();
         let stream = TcpStream::connect(address, &handle).map(move |stream| {
             channel.send(NetEvent::CONNECTED((event, id))).unwrap();
             Self::process_stream(event, id, handle, stream, connections, channel);
        });

        let stream = stream.map_err(|_|());
        let stream = stream.map(|_|());
        handle_out.spawn(stream);
        Ok(())
    }
    
    pub fn process_listen(handle: Handle,event: usize, id_source: Rc<Cell<usize>>, connections: Rc<RefCell<HashMap<usize, UnboundedSender<Vec<u8>>>>>, channel: UnboundedSender<NetEvent>, address: &SocketAddr) -> Result<()> {
        let handle_out = handle.clone();
        let mut listener = try!(TcpListener::bind(address, &handle));
        let srv = listener.incoming().for_each(move|(stream, _)|{
            let id = id_source.get();
            let channel_inner = channel.clone();
            id_source.set(id + 1);
            let handle_inner = handle.clone();
            let connections_inner = connections.clone();
            channel.send(NetEvent::CONNECTED((event, id))).unwrap();
            Self::process_stream(event, id, handle_inner, stream, connections_inner, channel_inner);
            Ok(())
        });

        let srv = srv.map_err(|_|());
        let srv = srv.map(|_|());
        
        handle_out.spawn(srv.then(move |_|{
            Ok(())
        }));
        Ok(())
    }

    pub fn process_timeout(handle: Handle, event: usize, sec: usize, channel: UnboundedSender<NetEvent>) -> Result<()>{
        let time = Duration::new(sec as u64, 0);
        let timeout = Timeout::new(time, &handle).unwrap();
        let timeout = timeout.map(move|_|{
            channel.send(NetEvent::TIMEOUT(event)).unwrap();
        });
        let timeout = timeout.map_err(|_|());
        let timeout = timeout.map(|_|());
        handle.spawn(timeout);
        Ok(())
    }

    pub fn process_send(id: usize, connections: Rc<RefCell<HashMap<usize, UnboundedSender<Vec<u8>>>>>, buf: Vec<u8>) {
        match connections.borrow_mut().get_mut(&id) {
            Some(tx) => {
                tx.send(buf).unwrap();
            },
            None => {
                println!("No Connection: {}!", id);
            },
        }
    }

    pub fn start(self) -> Result<(UnboundedSender<NetCommand>, UnboundedReceiver<NetEvent>)> {
        let (cmd_tx, cmd_rx) = mpsc::unbounded();
        let (event_tx, event_rx) = mpsc::unbounded();
        thread::spawn(move||{
            let mut core = Core::new().unwrap();
            let handle = core.handle();
            let handle_out = handle.clone();
            let id = Rc::new(Cell::new(0));
            let id_source = id.clone();
            let connections_out: Rc<RefCell<HashMap<usize, UnboundedSender<Vec<u8>>>>> =  Rc::new(RefCell::new(HashMap::new()));
            let connections = connections_out.clone();
            let event_tx_inner = event_tx.clone();
            let event_process = cmd_rx.fold(0, move |count, command|{
                let handle_inner = handle_out.clone();
                let id_inner = id_source.clone();
                let connections_inner = connections.clone();
                let channel_inner = event_tx_inner.clone();
                Self::process_command(handle_inner, id_inner, connections_inner, channel_inner, command);
                Ok(count + 1)
            }).map(|_|());
            
            core.run(event_process).unwrap();
        });
        Ok((cmd_tx, event_rx))
    }
}

