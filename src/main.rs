use std::net::{Ipv4Addr,TcpStream};
use std::thread;

const THREADS: u8 = 5;
const MAX_PORT: u16 = 65535;

fn try_connect(addr: Ipv4Addr, port: u16) {
    match TcpStream::connect((addr, port)) {
        Ok(_) => {
            println!("{} port is open", port);
        },
        Err(_) => {}
    }
}

fn main() {
    let localhost = Ipv4Addr::new(127, 0, 0, 1);
    let handles = (0..THREADS).map(move |_|{
        thread::spawn(move ||{
            for port in 0..MAX_PORT {
                try_connect(localhost,port+1);
            }
        })
    }).collect::<Vec<_>>();

    for handle in handles.into_iter() {
        handle.join().unwrap();
    }
}