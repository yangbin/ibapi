extern crate env_logger;
extern crate ibapi;

use std::env::args;

fn main() {
    env_logger::init();

    let args: Vec<_> = args().collect();

    let addr = args.get(1).map_or("127.0.0.1:7496", |s| s.as_str());

    ibapi::socket::Socket::connect(addr);
}

