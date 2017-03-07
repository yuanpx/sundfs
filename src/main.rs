mod storage;
use storage::nio::{NetService, NetEvent};
use storage::logic::{Service, Process};

struct TT {
}

impl Process for TT {
    fn process_init(&mut self, service: &mut Service) {
        let address = "127.0.0.1:9090".parse().unwrap();
        service.listen(address);
        let address = "127.0.0.1:9090".parse().unwrap();
        service.connect(address);
    }

    fn process_event(&mut self, service: &mut Service, ev: NetEvent) {
        match ev {
            NetEvent::CONNECTED(id) => {
                println!("connected id: {}", id);
            },
            NetEvent::DISCONNECTED(id)=> {
                println!("disconnected id: {}", id);
                
            },
            NetEvent::MESSAGE((id, buf)) => {
                println!("message id: {}", id);
            },
        }
    }
}

fn main() {
    println!("Hello, world!");
    let mut t = TT{};
    let mut s = Service::new();
    s.start(&mut t);
}
