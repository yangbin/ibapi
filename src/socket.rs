use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpStream, ToSocketAddrs};
use std::thread;

use log::{debug, error, info, warn};
use crossbeam_channel::{Receiver, Sender, unbounded};

use crate::protocol;
use crate::ib::{Hello, Message, Request};

const VERSION: &'static str = "twsapi_macunix.970.01";

const DEFAULT_HOST: &'static str = "127.0.0.1";
const DEFAULT_PORT: u64 = 7496;
const DEFAULT_CLIENT_ID: u64 = 0;

const CLIENT_VERSION: u64 = 71;
const SERVER_VERSION: u64 = 38;

pub struct Socket {
    pub rx: Receiver<Message>,
    pub tx: Sender<Request>,
    connected: bool,
}

impl Socket {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Socket {
        let mut stream = TcpStream::connect(addr).unwrap();

        info!("Connected");

        stream.write_all("API\0".as_bytes()).unwrap();

        let version = "v100..151".as_bytes();
        stream.write_all(&(version.len() as u32).to_be_bytes()).unwrap();
        stream.write_all(version).unwrap();

        let mut reader = BufReader::new(stream.try_clone().unwrap());

        let hello: Hello = protocol::from_reader(&mut reader).unwrap();
        info!("{:?}", hello);

        let req = Request::StartApi { client_id: 0, optional_capabilities: "".into() };
        protocol::to_writer(&mut stream, &req).unwrap();
        info!("Sent START_API");

        let (reader_tx, reader_rx) = unbounded();

        thread::Builder::new()
            .name("IB Socket Reader".into())
            .spawn(move|| read_loop(reader, reader_tx)).unwrap();

        let (writer_tx, writer_rx) = unbounded();

        thread::Builder::new()
            .name("IB Socket Writer".into())
            .spawn(move|| write_loop(stream, writer_rx)).unwrap();

        Socket {
            rx: reader_rx,
            tx: writer_tx,
            connected: false,
        }
    }
}

impl Socket {
    pub fn request(&self, request: Request) {
        self.tx.send(request).unwrap()
    }
}

fn read_loop<R: Read>(mut reader: BufReader<R>, tx: Sender<Message>) {
    loop {
        let msg: Result<Message, _> = protocol::from_reader(&mut reader);

        match msg {
            Err(err) => {
                let s = err.to_string();
                let mut s = s.split('`');

                match (s.next(), s.next(), s.next()) {
                    (Some("unknown variant "), Some(v), Some(", expected one of ")) => {
                        warn!("Unimplemented message ID: {}", v);
                        tx.send(Message::UnknownMessage(v.into())).unwrap();
                    },
                    _ => {
                        error!("Read error: {}", err);
                        break; // drop channel
                    }
                }
            }
            Ok(data) => {
                debug!("data: {:?}", data);
                tx.send(data).unwrap();
            }
        }
    }
}


fn write_loop<W: Write>(mut stream: W, rx: Receiver<Request>) {
    for request in rx {
        protocol::to_writer(&mut stream, &request).unwrap();
    }
}

impl <'a>Iterator for &'a Socket {
    type Item = Message;

    fn next(&mut self) -> Option<Self::Item> {
        self.rx.recv().ok()
    }
}
