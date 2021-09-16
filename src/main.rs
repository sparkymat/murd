use simple_logger::SimpleLogger;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

#[macro_use]
extern crate log;
extern crate simple_logger;

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
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

fn main() {
    SimpleLogger::new().init().unwrap();

    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    info!("Listening on 0.0.0.0:3333");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                info!("new connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                warn!("error: {}", e);
            }
        }
    }
    // close the socket server
    drop(listener);
}
