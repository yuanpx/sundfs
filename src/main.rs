mod storage;
use storage::nio::NetService;

fn main() {
    println!("Hello, world!");
    let mut s = NetService::new(); 
    let mut s = s.unwrap();
    let s = s.start();
}
