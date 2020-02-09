extern crate env_logger;
extern crate ibapi;

use std::env::args;

use ibapi::ib::{Contract, Message, Request};

fn main() {
    env_logger::init();

    let args: Vec<_> = args().collect();

    let addr = args.get(1).map_or("127.0.0.1:7496", |s| s.as_str());

    let ib = ibapi::socket::Socket::connect(addr);

    ib.request(Request::ReqPositions);

    for msg in ib.rx {
        use Message::*;

        match msg {
            PositionData { account: acc, contract, position, avg_cost, .. } => {
                let sym = contract.local_symbol;
                let val = position * avg_cost;
                println!("{} {:22} {:8} {:8.2} {:10.2}", acc, sym, position, avg_cost, val);
            },
            PositionDataEnd { .. } => break,
            _ => {
                println!(">>> [IB] {:?}", msg);
            }
        }
    }
}
