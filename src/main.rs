use simple_logger::SimpleLogger;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

#[macro_use]
extern crate log;
extern crate serde;
extern crate simple_logger;

mod config;

fn handle_client(mut stream: TcpStream) {
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

fn main() {
    SimpleLogger::new().init().unwrap();

    let cfg = config::load();
    info!("config = {:#?}", cfg);

    let listener = TcpListener::bind(cfg.listen_address()).unwrap();
    info!("Listening on {}", cfg.listen_address());

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
