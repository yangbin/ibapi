use std::default::Default;
use std::io::BufRead;
use std::io::Error as IoError;
use std::error::Error;
use std::fmt;
use std::convert::From;

use serde;
use serde::de::Visitor;
use serde::de::value::ValueDeserializer;
use serde::de::value::Error as SerdeError;

const EOL: u8 = b'\0';

fn convert_io_error(e: IoError) -> SerdeError {
    SerdeError::Custom(e.description().into())
}

pub type DeserializeResult<T> = Result<T, SerdeError>;

/// Deserializes IB's API protocol. Each field regardless of type is
/// represented as a null-terminated string.
pub struct Deserializer<'a, R: 'a> {
    reader: &'a mut R,
    next: u8,
}

impl<'a, R: BufRead> Deserializer<'a, R> {
    pub fn new(r: &'a mut R) -> Deserializer<'a, R> {
        Deserializer {
            reader: r,
            next: 0,
        }
    }

    fn read_string(&mut self) -> DeserializeResult<String> {
        let mut buffer = Vec::new();

        try!(self.reader.read_until(EOL, &mut buffer).map_err(convert_io_error));
        buffer.pop();

        debug!(">>> {}", String::from_utf8_lossy(&buffer));
        Ok(String::from_utf8_lossy(&buffer).into_owned())
    }
}

impl<'a, R: BufRead> serde::Deserializer for Deserializer<'a, R> {
    type Error = SerdeError;

    fn deserialize<V>(&mut self, visitor: V) -> DeserializeResult<V::Value>
        where V: Visitor
    {
        panic!("Deserializer::deserialize is not supported")
    }

    fn deserialize_u64<V>(&mut self, mut visitor: V) -> DeserializeResult<V::Value>
        where V: serde::de::Visitor
    {
        let s = try!(self.read_string());

        visitor.visit_u64(s.parse().unwrap())
    }

    fn deserialize_str<V>(&mut self, mut visitor: V) -> DeserializeResult<V::Value>
        where V: serde::de::Visitor
    {
        visitor.visit_str(&try!(self.read_string()))
    }

    fn deserialize_string<V>(&mut self, mut visitor: V) -> DeserializeResult<V::Value>
        where V: serde::de::Visitor
    {
        visitor.visit_string(try!(self.read_string()))
    }

    fn deserialize_struct<V>(&mut self,
                             _name: &str,
                             fields: &'static [&'static str],
                             mut visitor: V)
                             -> DeserializeResult<V::Value>
        where V: serde::de::Visitor
    {
        struct StructVisitor<'a, 'b: 'a, R: BufRead + 'b>(&'a mut Deserializer<'b, R>);

        impl<'a, 'b: 'a, R: BufRead + 'b> serde::de::SeqVisitor for StructVisitor<'a, 'b, R> {
            type Error = SerdeError;

            fn visit<T>(&mut self) -> Result<Option<T>, Self::Error>
                where T: serde::de::Deserialize
            {
                let value = try!(serde::Deserialize::deserialize(self.0));
                Ok(Some(value))
            }

            fn end(&mut self) -> Result<(), Self::Error> {
                Ok(())
            }
        }

        visitor.visit_seq(StructVisitor(self))
    }

    forward_to_deserialize! {
        bool usize u8 u16 u32 isize i8 i16 i32 i64 f32 f64 char
        unit option seq seq_fixed_size bytes map unit_struct newtype_struct
        tuple_struct struct_field tuple enum ignored_any
    }
}
