use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpStream, ToSocketAddrs};

use log::{debug, error};

use serde::{Serialize, Deserialize};

use crate::protocol;

const VERSION: &'static str = "twsapi_macunix.970.01";

const DEFAULT_HOST: &'static str = "127.0.0.1";
const DEFAULT_PORT: u64 = 7496;
const DEFAULT_CLIENT_ID: u64 = 0;

const CLIENT_VERSION: u64 = 71;
const SERVER_VERSION: u64 = 38;

const EOL: u8 = b'\0';

pub struct Socket {
    connected: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct Packet {
    version: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Hello {
    server_version: u64,
    server_connection_time: String,
}

impl Socket {
    pub fn connect<A: ToSocketAddrs>(addr: A) {
        let mut stream = TcpStream::connect(addr).unwrap();

        stream.write_all(format!("{}\0", CLIENT_VERSION).as_bytes()).unwrap();
        stream.write_all(format!("{}\0", 1).as_bytes()).unwrap();

        let mut reader = BufReader::new(stream.try_clone().unwrap());

        let hello: Hello = protocol::deserialize_from(&mut reader).unwrap();

        debug!("hello: {:?}", hello);

        // Read loop
        loop {
            match protocol::deserialize_from(&mut reader) {
                Err(err) => {
                    error!("Error: {}", err);
                    break;
                }
                Ok(data) => {
                    let data: String = data;

                    debug!("data: {:?}", data);
                }
            }
        }
    }
}
