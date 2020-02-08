use std::fmt::Display;
use std::io::prelude::*;

use serde::{self, Deserialize, Serialize};
use serde::de::value::Error as DeError;

use self::de::Deserializer;
use self::ser::Serializer;

pub mod de;
pub mod ser;

/// Deserializes directly from a `Buffer`ed Reader.
///
/// If any error occurs, assume the buffer is in an invalid state, as the IB
/// protocol cannot be re-synchronized.
pub fn from_reader<'a, R, T>(reader: &'a mut R) -> Result<T, DeError>
    where R: BufRead,
          T: Deserialize<'a>
{
    let mut deserializer = Deserializer::new_v100plus(reader)?;

    serde::Deserialize::deserialize(&mut deserializer)
}

/// Serializes an object directly into a `Writer`.
///
/// Since we need to prefix with the message length, the serializer will
/// serialize into a String first.
pub fn to_writer<W, T: ?Sized>(writer: &mut W, value: &T) -> Result<(), Error>
where
    W: std::io::Write,
    T: Serialize,
{
    let msg = to_bytes(value)?;
    let len = msg.len() as u32;
    writer.write_all(&len.to_be_bytes())?;
    writer.write_all(&msg)?;
    Ok(())
}

pub fn to_bytes<T: ?Sized + Serialize>(value: &T) -> Result<Vec<u8>, Error> {
    let mut serializer = Serializer::new();
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    SequenceMustHaveLength,
    /// A custom error message from Serde.
    Custom(String),
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref err) => std::error::Error::description(err),
            Error::SequenceMustHaveLength => "sequence / map must have known length",
            Error::Custom(ref msg) => msg,
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            Error::Io(ref err) => Some(err),
            Error::SequenceMustHaveLength => None,
            Error::Custom(_) => None,
        }
    }
}


impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err).into()
    }
}

impl Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::Io(ref ioerr) => write!(fmt, "IO error: {}", ioerr),
            Error::SequenceMustHaveLength => write!(fmt, "{}", self),
            Error::Custom(ref s) => s.fmt(fmt),
        }
    }
}
impl serde::de::Error for Error {
    fn custom<T: Display>(desc: T) -> Error {
        Error::Custom(desc.to_string()).into()
    }
}

impl serde::ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Error {
        Error::Custom(msg.to_string()).into()
    }
}
