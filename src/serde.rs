use crate::{BYTES_PER_WORD, BitVec};
use alloc::vec::Vec;
use core::fmt;
use serde::de::{Deserialize, Deserializer, Error, MapAccess, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeStruct, Serializer};

const FIELDS: &[&str] = &["len", "data"];
const BYTES_INSUFFICIENT: &str = "bytes insufficient for given bit length";

impl Serialize for BitVec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bytes = Self::bytes_required(self.len);
        let (div, rem) = (bytes / BYTES_PER_WORD, bytes % BYTES_PER_WORD);
        let mut data = Vec::with_capacity(bytes);
        for i in 0..div {
            let word = unsafe { self.data.get_unchecked(i) };
            data.extend(word.to_be_bytes());
        }
        let word = unsafe { self.data.get_unchecked(div) };
        data.extend_from_slice(&word.to_be_bytes()[..rem]);

        let mut vec = serializer.serialize_struct("BitVec", 2)?;
        vec.serialize_field("len", &self.len)?;
        vec.serialize_field("data", &data)?;
        vec.end()
    }
}

impl<'de> Deserialize<'de> for BitVec {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct("BitVec", FIELDS, BitVecVisitor)
    }
}

#[derive(Debug)]
struct BitVecVisitor;

impl<'de> Visitor<'de> for BitVecVisitor {
    type Value = BitVec;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct BitVec")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let len: usize = seq
            .next_element()?
            .ok_or_else(|| Error::invalid_length(0, &self))?;
        let data: Vec<u8> = seq
            .next_element()?
            .ok_or_else(|| Error::invalid_length(1, &self))?;

        let bytes = Self::Value::bytes_required(len);
        if data.len() < bytes {
            Err(A::Error::custom(BYTES_INSUFFICIENT))
        } else {
            let slice = unsafe { data.get_unchecked(..bytes) };
            let mut vec = Self::Value::from_bytes(slice);
            vec.len = len;
            Ok(vec)
        }
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut len: Option<usize> = None;
        let mut data: Option<Vec<u8>> = None;

        while let Some(key) = map.next_key()? {
            match key {
                Field::Len => {
                    if len.is_some() {
                        return Err(Error::duplicate_field("len"));
                    }
                    len = Some(map.next_value()?);
                }
                Field::Data => {
                    if data.is_some() {
                        return Err(Error::duplicate_field("data"));
                    }
                    data = Some(map.next_value()?);
                }
            }
        }

        let len = len.ok_or_else(|| Error::missing_field("len"))?;
        let data = data.ok_or_else(|| Error::missing_field("data"))?;

        let bytes = Self::Value::bytes_required(len);
        if data.len() < bytes {
            Err(A::Error::custom(BYTES_INSUFFICIENT))
        } else {
            let slice = unsafe { data.get_unchecked(..bytes) };
            let mut vec = Self::Value::from_bytes(slice);
            vec.len = len;
            Ok(vec)
        }
    }
}

#[derive(Debug)]
enum Field {
    Len,
    Data,
}

impl<'de> Deserialize<'de> for Field {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_identifier(FieldVisitor)
    }
}

#[derive(Debug)]
struct FieldVisitor;

impl Visitor<'_> for FieldVisitor {
    type Value = Field;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("`len` or `data`")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match value {
            "len" => Ok(Self::Value::Len),
            "data" => Ok(Self::Value::Data),
            _ => Err(E::unknown_field(value, FIELDS)),
        }
    }
}
