use bincode::ErrorKind;
use bincode::Options;
use clvmr::allocator::Allocator;
use clvmr::node::Node;
use clvmr::serialize::{node_from_bytes, node_to_bytes};
use core::fmt::Formatter;
use serde::de::Error as Derror;
use serde::de::{DeserializeSeed, SeqAccess, Visitor};
use serde::ser::{SerializeTuple, StdError};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::error::Error;
use std::fmt::{Debug, Display};

//use pyo3::types::PyBytes;

#[derive(Debug)]
pub struct Program {}

#[derive(Debug)]
pub struct ProgramArray(Vec<u8>);

struct BufferVisitor {
    amount: usize,
}

#[derive(Debug)]
enum MyError {}

struct SeqProgramBytes {}

impl Derror for MyError {
    fn custom<T>(_: T) -> Self
    where
        T: std::fmt::Display,
    {
        todo!()
    }
}
impl StdError for MyError {}

impl Display for MyError {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        todo!()
    }
}

impl<'de> SeqAccess<'de> for SeqProgramBytes {
    type Error = MyError;
    fn next_element_seed<T>(
        &mut self,
        _: T,
    ) -> Result<
        std::option::Option<<T as DeserializeSeed<'de>>::Value>,
        <Self as SeqAccess<'de>>::Error,
    >
    where
        T: DeserializeSeed<'de>,
    {
        todo!()
    }
}

/*
impl<'a> Visitor<'a> for BufferVisitor {
    type Value = Vec<u8>;

    fn visit_seq<SeqProgramBytes>(
        self,
        seq: SeqProgramBytes,
    ) -> Result<Self::Value, <SeqProgramBytes as SeqAccess<'a>>::Error> {
        dbg!("{:?}", seq);
        todo!()
    }

    fn expecting(
        &self,
        _: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}
*/

impl Serialize for Program {
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        todo!()
    }
}

/*
impl<'de> Deserialize<'de> for ProgramArray {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let buffer_visitor = BufferVisitor { amount: 1 };
        let r = deserializer.deserialize_seq(buffer_visitor);
        dbg!(&r);
        let p = r.unwrap();
        Ok(ProgramArray(p))
    }
}
*/

/*
impl<'de> DeserializeSeed<'de> for Program {
    type Value = Vec<u8>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        let buffer_visitor = BufferVisitor { amount: 1 };
        let r = deserializer.deserialize_u8(buffer_visitor);
        dbg!(r);
        r
    }
}*/

/*
#[test]
fn test_deserialize_program() {
    let chia = bincode::DefaultOptions::new()
        .with_chia_int_encoding()
        .allow_trailing_bytes()
        .with_big_endian();
    let blob = hex::decode("80").unwrap();
    dbg!(&blob);
    let p: Result<ProgramArray, _> = chia.deserialize(&blob);
    dbg!(&p);
    p.unwrap();
}
*/

#[derive(Debug)]
struct MyNode<'a>(Node<'a>);

impl Serialize for MyNode<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let r = node_to_bytes(&self.0).unwrap();
        let len = r.len();
        let mut st = serializer.serialize_tuple(len)?;
        for c in r {
            st.serialize_element(&c)?;
        }
        st.end()
    }
}

#[test]
fn test_serialize_node() {
    let chia = bincode::DefaultOptions::new()
        .with_chia_int_encoding()
        .allow_trailing_bytes()
        .with_big_endian();
    let blob = hex::decode("ff08ff0aff1480").unwrap();
    let mut allocator = Allocator::new();
    let node_ptr = node_from_bytes(&mut allocator, &blob).unwrap();
    let node = MyNode(Node::new(&mut allocator, node_ptr));
    dbg!(&node);
    let p = chia.serialize(&node);
    dbg!(&p);
    //p.unwrap();
    todo!();
}
