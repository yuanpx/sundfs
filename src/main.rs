mod storage;
use storage::nio::{NetService, NetEvent};
use storage::logic::{Service, Process};

struct TT {
}

impl Process for TT {
    fn process_init(&mut self, service: &mut Service) {
        let address = "127.0.0.1:9090".parse().unwrap();
        service.listen(0,address);
        let address = "127.0.0.1:9090".parse().unwrap();
        service.connect(0,address);
        service.timeout(10, 5);
    }

    fn process_event(&mut self, service: &mut Service, ev: NetEvent) {
        match ev {
            NetEvent::CONNECTED((event,id)) => {
                println!("connected id: {}", id);
            },
            NetEvent::DISCONNECTED((event,id))=> {
                println!("disconnected id: {}", id);
                
            },
            NetEvent::MESSAGE((id, buf)) => {
                println!("message id: {}", id);
            },
            NetEvent::TIMEOUT(event) => {
                println!("timeout id: {}", event);
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    let mut t = TT{};
    let mut s = Service::new();
    s.start(&mut t);
}
