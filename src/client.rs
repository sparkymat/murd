use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};

pub fn handle(mut stream: TcpStream) {
    let mut data = [0 as u8; 1024];
    while match stream.read(&mut data) {
        Ok(size) => {
            stream.write(&data[0..size]).unwrap();
            true
        }
        Err(_) => {
            warn!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}
