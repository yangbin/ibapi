use std::io::prelude::*;

use serde;
use serde::de::value::Error;

use self::de::Deserializer;

pub mod de;

/// Deserializes directly from a `Buffer`ed Reader.
///
/// If any error occurs, assume the buffer is in an invalid state, as the IB
/// protocol cannot be re-synchronized.
pub fn deserialize_from<'a, R, T>(reader: &'a mut R) -> Result<T, Error>
    where R: BufRead,
          T: serde::Deserialize<'a>
{
    let mut deserializer = Deserializer::new(reader);

    serde::Deserialize::deserialize(&mut deserializer)
}
