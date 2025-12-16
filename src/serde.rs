use crate::BitVec;
use crate::primitive::{Byte, Word};
use alloc::vec::Vec;
use core::fmt;
use serde::de::{Deserialize, Deserializer, Error, MapAccess, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeSeq, SerializeStruct, Serializer};

const FIELDS: &[&str] = &["len", "buf"];

impl Serialize for BitVec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut vec = serializer.serialize_struct("BitVec", 2)?;
        vec.serialize_field("len", &self.len)?;
        vec.serialize_field("buf", &BufProxy(self))?;
        vec.end()
    }
}

#[derive(Debug)]
struct BufProxy<'a>(&'a BitVec);

impl Serialize for BufProxy<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let seq_len = self.0.len.div_ceil(Byte::BITS as usize);
        let head_words = seq_len / Word::BYTES;
        let tail_bytes = seq_len % Word::BYTES;
        let mut seq = serializer.serialize_seq(Some(seq_len))?;

        let head = unsafe { self.0.buf.get_unchecked(..head_words) };
        for word in head {
            let word = word.to_byte_array();
            for byte in word {
                seq.serialize_element(&byte)?;
            }
        }

        if tail_bytes != 0 {
            let word = unsafe { self.0.buf.get_unchecked(head_words) };
            let word = word.to_byte_array();
            let tail = unsafe { word.get_unchecked(0..tail_bytes) };
            for byte in tail {
                seq.serialize_element(&byte)?;
            }
        }

        seq.end()
    }
}

impl<'de> Deserialize<'de> for BitVec {
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
        let buf: Vec<Byte> = seq
            .next_element()?
            .ok_or_else(|| Error::invalid_length(1, &self))?;

        let mut vec = BitVec::from(buf);
        vec.len = vec.len.min(len);

        Ok(vec)
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut len: Option<usize> = None;
        let mut buf: Option<Vec<Byte>> = None;

        while let Some(key) = map.next_key()? {
            match key {
                Field::Len => {
                    if len.is_some() {
                        return Err(Error::duplicate_field("len"));
                    }
                    len = Some(map.next_value()?);
                }
                Field::Buf => {
                    if buf.is_some() {
                        return Err(Error::duplicate_field("buf"));
                    }
                    buf = Some(map.next_value()?);
                }
            }
        }

        let len = len.ok_or_else(|| Error::missing_field("len"))?;
        let buf = buf.ok_or_else(|| Error::missing_field("buf"))?;

        let mut vec = BitVec::from(buf);
        vec.len = vec.len.min(len);

        Ok(vec)
    }
}

#[derive(Debug)]
enum Field {
    Len,
    Buf,
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
        formatter.write_str("`len` or `buf`")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match value {
            "len" => Ok(Field::Len),
            "buf" => Ok(Field::Buf),
            _ => Err(Error::unknown_field(value, FIELDS)),
        }
    }
}
