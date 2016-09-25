#![feature(custom_derive)]
#![feature(plugin)]

#![plugin(serde_macros)]

extern crate env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

pub mod protocol;
pub mod socket;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
