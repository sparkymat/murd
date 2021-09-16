use serde::{Deserialize, Serialize};
use simple_logger::SimpleLogger;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

#[macro_use]
extern crate log;
extern crate serde;
extern crate simple_logger;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    listen_host: String,
    listen_port: u32,
}

impl Config {
    fn listen_address(&self) -> String {
        format!("{}:{}", self.listen_host, self.listen_port)
    }
}

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

fn load_config() -> Config {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("config")).unwrap();

    // Print out our settings (as a HashMap)
    return settings.try_into::<Config>().unwrap();
}

fn main() {
    SimpleLogger::new().init().unwrap();

    let config = load_config();
    info!("config = {:#?}", config);

    let listener = TcpListener::bind(config.listen_address()).unwrap();
    info!("Listening on {}", config.listen_address());

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
