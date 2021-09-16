use simple_logger::SimpleLogger;
use std::net::TcpListener;
use std::thread;

#[macro_use]
extern crate log;
extern crate serde;
extern crate simple_logger;

mod client;
mod config;

fn main() {
    SimpleLogger::new().init().unwrap();

    let cfg = config::load();
    let listener = TcpListener::bind(cfg.listen_address()).unwrap();
    info!("Listening on {}", cfg.listen_address());

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                info!("new connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    // connection succeeded
                    client::handle(stream)
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
