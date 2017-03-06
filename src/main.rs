mod storage;
use storage::nio::NetService;

fn main() {
    println!("Hello, world!");
    let mut s = NetService::new("127.0.0.1:9000"); 
    let mut s = s.unwrap();
    let s = s.start();
}
