use std::io::BufRead;
use std::io::Error as IoError;

use log::debug;
use serde::{self, forward_to_deserialize_any};
use serde::de::Visitor;
use serde::de::Error as DeError;
use serde::de::value::Error as SerdeError;

const EOL: u8 = b'\0';

fn convert_io_error(e: IoError) -> SerdeError {
    SerdeError::custom(e)
}

pub type DeserializeResult<T> = Result<T, SerdeError>;

/// Deserializes IB's API protocol. Each field regardless of type is
/// represented as a null-terminated string.
pub struct Deserializer<R> {
    reader: R,
}

impl<R: BufRead> Deserializer<R> {
    pub fn new(r: R) -> Deserializer<R> {
        Deserializer {
            reader: r,
        }
    }

    fn read_string(&mut self) -> DeserializeResult<String> {
        let mut buffer = Vec::new();

        let len = self.reader.read_until(EOL, &mut buffer).map_err(convert_io_error)?;

        if len == 0 {
            return Err(SerdeError::custom("EOF"))
        }

        buffer.pop();

        debug!(">>> {}", String::from_utf8_lossy(&buffer));
        Ok(String::from_utf8_lossy(&buffer).into_owned())
    }
}

impl<'de, 'a, R: BufRead> serde::Deserializer<'de> for &'a mut Deserializer<R> {
    type Error = SerdeError;

    fn deserialize_any<V>(self, visitor: V) -> DeserializeResult<V::Value>
        where V: Visitor<'de>
    {
        panic!("Deserializer::deserialize_any is not supported")
    }

    fn deserialize_u64<V>(self, visitor: V) -> DeserializeResult<V::Value>
    where
        V: Visitor<'de>
    {
        let s = self.read_string()?;

        visitor.visit_u64(s.parse().unwrap())
    }

    fn deserialize_str<V>(self, visitor: V) -> DeserializeResult<V::Value>
        where V: serde::de::Visitor<'de>
    {
        visitor.visit_str(&self.read_string()?)
    }

    fn deserialize_string<V>(self, visitor: V) -> DeserializeResult<V::Value>
        where V: serde::de::Visitor<'de>
    {
        visitor.visit_string(self.read_string()?)
    }

    fn deserialize_struct<V>(self,
                             _name: &str,
                             fields: &'static [&'static str],
                             visitor: V)
                             -> DeserializeResult<V::Value>
        where V: serde::de::Visitor<'de>
    {
        self.deserialize_tuple(fields.len(), visitor)
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> DeserializeResult<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        struct Access<'a, R: BufRead> {
            deserializer: &'a mut Deserializer<R>,
            len: usize,
        }

        impl<'de, 'a, 'b: 'a, R: BufRead + 'b> serde::de::SeqAccess<'de>
            for Access<'a, R>
        {
            type Error = SerdeError;

            fn next_element_seed<T>(&mut self, seed: T) -> DeserializeResult<Option<T::Value>>
            where
                T: serde::de::DeserializeSeed<'de>,
            {
                if self.len > 0 {
                    self.len -= 1;
                    let value = serde::de::DeserializeSeed::deserialize(
                        seed,
                        &mut *self.deserializer,
                    )?;
                    Ok(Some(value))
                } else {
                    Ok(None)
                }
            }

            fn size_hint(&self) -> Option<usize> {
                Some(self.len)
            }
        }

        visitor.visit_seq(Access {
            deserializer: self,
            len: len,
        })
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u128 f32 f64 char
        bytes byte_buf option unit unit_struct newtype_struct seq //tuple
        tuple_struct map enum identifier ignored_any
    }
}
