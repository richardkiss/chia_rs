use bincode::Options;
use clvmr::node::Node;
use clvmr::serialize::node_from_bytes;
use serde::de::{DeserializeSeed, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::error::Error;
use std::fmt::Debug;

//use pyo3::types::PyBytes;

#[derive(Debug)]
pub struct Program {}

struct BufferVisitor {
    amount: usize,
}

impl Visitor<'_> for BufferVisitor {
    type Value = Program;

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        println!("{:?}", v);
        todo!()
    }

    fn expecting(
        &self,
        _: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl Serialize for Program {
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        todo!()
    }
}

impl<'de> Deserialize<'de> for Program {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let buffer_visitor = BufferVisitor { amount: 1 };
        let r = deserializer.deserialize_u8(buffer_visitor);
        println!("{:?}", r);
        r
    }
}

impl<'de> DeserializeSeed<'de> for Program {
    type Value = Self;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        let buffer_visitor = BufferVisitor { amount: 1 };
        let r = deserializer.deserialize_u8(buffer_visitor);
        println!("{:?}", r);
        r
    }
}

#[test]
fn test_deserialize_program() {
    let chia = bincode::DefaultOptions::new()
        .with_chia_int_encoding()
        .allow_trailing_bytes()
        .with_big_endian();
    let blob = hex::decode("80").unwrap();
    println!("{:?}", blob);
    let p: Result<Program, _> = chia.deserialize(&blob);
    println!("{:?}", p);
    p.unwrap();
}
