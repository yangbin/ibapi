use std::fmt::Debug;
use std::io::{BufRead, Cursor};
use std::io::Error as IoError;
use std::str::FromStr;

use log::{debug, error};
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
    peek: Option<Vec<u8>>,
}

impl<R: BufRead> Deserializer<R> {
    pub fn new(r: R) -> Deserializer<R> {
        Deserializer {
            reader: r,
            peek: None,
        }
    }

    pub fn new_v100plus(r: &mut R) -> Result<Deserializer<Cursor<Vec<u8>>>, SerdeError> {
        let mut len = [0; 4];

        r.read_exact(&mut len)
            .map_err(convert_io_error)?;

        let len = u32::from_be_bytes(len);

        let mut buffer = vec![0; len as usize];

        r.read_exact(&mut buffer)
            .map_err(convert_io_error)?;

        Ok(Deserializer::new(Cursor::new(buffer)))
    }

    fn decode_field(&mut self) -> DeserializeResult<Vec<u8>> {
        let mut buffer = Vec::new();
        let len = self.reader.read_until(EOL, &mut buffer)
            .map_err(convert_io_error)?;

        if len == 0 {
            return Err(SerdeError::custom("EOF"))
        }

        buffer.pop(); // throw away EOL

        debug!(">>> {:?}", buffer);
        debug!(" >> {}", std::str::from_utf8(&buffer).unwrap_or_default());

        Ok(buffer)
    }

    fn peek<'a>(&'a mut self) -> DeserializeResult<&'a [u8]> {
        self.peek = Some(self.decode_field()?);
        Ok(self.peek.as_deref().unwrap())
    }

    fn discard_peek(&mut self) {
        self.peek = None;
    }

    fn read_field<T: FromStr + Debug>(&mut self) -> DeserializeResult<T> {
        let buffer = match self.peek.take() {
            Some(b) => b,
            None => self.decode_field()?
        };

        let parsed = std::str::from_utf8(&buffer)
            .map_err(SerdeError::custom)?
            .parse()
            .map_err(|_| SerdeError::custom(format!("Parse error: expected {}", std::any::type_name::<T>())))?;

        debug!("  > {:?}", parsed);

        Ok(parsed)
    }
}

macro_rules! deserialize_parsable {
    ($deserialize_method:ident, $visit_method:ident) => {
        #[inline]
        fn $deserialize_method<V>(self, visitor: V) -> DeserializeResult<V::Value>
            where V: Visitor<'de>,
        {
            visitor.$visit_method(self.read_field()?)
        }
    }
}

macro_rules! deserialize_unimplemented {
    ($deserialize_method:ident) => {
        fn $deserialize_method<V>(self, _visitor: V) -> DeserializeResult<V::Value>
            where V: Visitor<'de>,
        {
            error!(stringify!($deserialize_method, "not implemented"));
            unimplemented!()
        }
    }
}

impl<'de, 'a, R: BufRead> serde::Deserializer<'de> for &'a mut Deserializer<R> {
    type Error = SerdeError;

    deserialize_parsable!(deserialize_i8, visit_i8);
    deserialize_parsable!(deserialize_i16, visit_i16);
    deserialize_parsable!(deserialize_i32, visit_i32);
    deserialize_parsable!(deserialize_i64, visit_i64);
    deserialize_parsable!(deserialize_i128, visit_i128);
    deserialize_parsable!(deserialize_u8, visit_u8);
    deserialize_parsable!(deserialize_u16, visit_u16);
    deserialize_parsable!(deserialize_u32, visit_u32);
    deserialize_parsable!(deserialize_u64, visit_u64);
    deserialize_parsable!(deserialize_u128, visit_u128);
    deserialize_parsable!(deserialize_f32, visit_f32);
    deserialize_parsable!(deserialize_f64, visit_f64);
    deserialize_parsable!(deserialize_string, visit_string);

    fn deserialize_bool<V>(self, visitor: V) -> DeserializeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bool(self.read_field::<i32>()? != 0)
    }

    fn deserialize_str<V>(self, visitor: V) -> DeserializeResult<V::Value>
        where V: Visitor<'de>
    {
        visitor.visit_str(&self.read_field::<String>()?)
    }

    fn deserialize_enum<V>( self,
                             _name: &'static str,
                             _variants: &'static [&'static str],
                             visitor: V)
                             -> DeserializeResult<V::Value>
        where V: Visitor<'de>
    {
        visitor.visit_enum(self)
    }

    fn deserialize_struct<V>(self,
                             _name: &str,
                             fields: &'static [&'static str],
                             visitor: V)
                             -> DeserializeResult<V::Value>
        where V: Visitor<'de>
    {
        self.deserialize_tuple(fields.len(), visitor)
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> DeserializeResult<V::Value>
    where
        V: Visitor<'de>,
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
                }
                else {
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

    fn deserialize_seq<V>(self, visitor: V) -> DeserializeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        let len = serde::Deserialize::deserialize(&mut *self)?;

        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_unit<V>(self, visitor: V) -> DeserializeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> DeserializeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> DeserializeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> DeserializeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!();
    }

    fn deserialize_option<V>(self, visitor: V) -> DeserializeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.peek()? {
            b"" | // Option<String> or Option<(String, ...)> etc
            b"2147483647" |           // std::i32::MAX
            b"9223372036854775807" |  // std::i64::MAX
            b"1.7976931348623157E308" // std::f64::MAX
                => {
                    self.discard_peek();
                    visitor.visit_none()
                },
            _ => visitor.visit_some(&mut *self),
        }
    }

    deserialize_unimplemented!(deserialize_any);
    deserialize_unimplemented!(deserialize_char);
    deserialize_unimplemented!(deserialize_bytes);
    deserialize_unimplemented!(deserialize_byte_buf);
    deserialize_unimplemented!(deserialize_map);
    deserialize_unimplemented!(deserialize_identifier);
    deserialize_unimplemented!(deserialize_ignored_any);
}

impl<'de, 'a, R: BufRead + 'a> serde::de::EnumAccess<'de> for &'a mut Deserializer<R>
{
    type Error = SerdeError;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> DeserializeResult<(V::Value, Self::Variant)>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        use serde::de::IntoDeserializer;

        let id: String = serde::Deserialize::deserialize(&mut *self)?;
        let val: DeserializeResult<_> = seed.deserialize(id.into_deserializer());
        Ok((val?, self))
    }
}

impl<'de, 'a, R> serde::de::VariantAccess<'de> for &'a mut Deserializer<R>
where
    R: BufRead,
{
    type Error = SerdeError;

    fn unit_variant(self) -> DeserializeResult<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> DeserializeResult<T::Value>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        serde::de::DeserializeSeed::deserialize(seed, self)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> DeserializeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        serde::de::Deserializer::deserialize_tuple(self, len, visitor)
    }

    fn struct_variant<V>(self, fields: &'static [&'static str], visitor: V) -> DeserializeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        serde::de::Deserializer::deserialize_tuple(self, fields.len(), visitor)
    }
}
