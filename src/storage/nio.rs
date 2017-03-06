extern crate futures;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::iter;
use std::env;
use std::io::Result;
use std::net::SocketAddr;
use std::io::{Error, ErrorKind};
use std::time::Duration;
extern crate tokio_core;
use self::tokio_core::net::TcpListener;
use self::tokio_core::net::TcpStream;
use self::tokio_core::reactor::Core;
use self::tokio_core::reactor::Timeout;
use self::tokio_core::reactor::Interval;
use self::tokio_core::io;
use self::tokio_core::io::Io;
use self::futures::stream;
use self::futures::stream::Stream;
use self::futures::Future;


pub struct NetService {
    pub address: SocketAddr,
    pub  core: Core,
//    pub  listener: Rc<TcpListener>,
    //pub  connections: Rc<HashMap<String, TcpStream>>,
}

impl NetService{
    pub fn new(address: &str) -> Result<NetService> {
        //let addr = try!(address.to_string().parse());
        let addr = address.to_string().parse().unwrap();
        let mut core = try!(Core::new());
        Ok(NetService{
            address: addr,
            core: core,
 //           listener: Rc::new(listener),
     //       connections: Rc::new(HashMap::new()),
        })
    }

    pub fn connect(&mut self, address: &SocketAddr) -> Result<()> {
       // let handle = self.core.handle();
       // let stream = TcpStream::connect(address, &handle);
        Ok(())
    }

    pub fn listen(&mut self) -> Result<()> {
        let handle = self.core.handle();
        let mut listener = try!(TcpListener::bind(&self.address, &handle));
        let srv = listener.incoming().for_each(move|(stream, addr)|{
            println!("New Connection: {}", addr);
            let (reader, writer) = stream.split();
       //     let (tx, rx) = futures::sync::mpsc::unbounded();
      //      self.connections.insert(addr, tx);
      //      let connections_inner = self.connections.clone();

            let iter = stream::iter(iter::repeat(()).map(Ok::<(), Error>));
            //let mut header_vec: Vec<u8> = vec![0;4];
            let mut header_vec: Vec<u8> = vec![0;1];
            let socket_reader = iter.fold((reader, header_vec), move | (reader, header_vec), _| {
                let header = io::read_exact(reader, header_vec);
                let header = header.and_then(|(reader, header_vec)|{
                    if header_vec.len() == 0 {
                        Err(Error::new(ErrorKind::BrokenPipe, "broken pipe"))
                    } else {

                        Ok((reader, header_vec))
                    }
                });

                let body = header.and_then(|(reader, header_vec)|{
                    let body_len = header_vec[0] as usize; 
                    let mut body_vec = vec![0; body_len];
                    let body_inner = io::read_exact(reader, body_vec);
                    let body_inner = body_inner.and_then(|(reader, body_vec)|{
                        if body_vec.len() == 0 {
                            Err(Error::new(ErrorKind::BrokenPipe, "broken pipe"))
                        } else {
                            Ok((reader, header_vec, body_vec))
                        }
                    });
                    body_inner
                });

                //let body = body.map(|(reader, vec)| {
                body.map(|(reader, header_vec, body_vec)| {
                   // (reader, String::from_utf8(vec))
                    println!("{:?}", String::from_utf8(body_vec));
                    (reader, header_vec)
                })
            });

            let socket_reader = socket_reader.map_err(|_| ());
            let connection = socket_reader.map(|_|());
            handle.spawn(connection.then(move |_|{
                println!("Connection {} closed.", addr);
                Ok(())
            }));

            Ok(())
        });

        let srv = srv.map_err(|_|());
        let srv = srv.map(|_|());
        
        let handle = self.core.handle();
        handle.spawn(srv.then(move |_|{
            Ok(())
        }));

//        self.core.run(srv).unwrap();
        Ok(())
    }

    pub    fn start(&mut self) -> Result<()> {
        let handle = self.core.handle();
        let iter = stream::iter(iter::repeat(()).map(Ok::<(), Error>));
        let timer = iter.fold(0, move |count, _| {
            let duration = Duration::new(1, 0);
            let time_out = Timeout::new(duration, &handle).unwrap();
            time_out.map(move |_|{
                println!("tick 1s: count {}!", count);
                count + 1
            })
        });
        
        self.core.run(timer).unwrap();
        Ok(())
    }
}
