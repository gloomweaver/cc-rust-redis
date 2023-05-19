use anyhow::Result;
use bytes::BytesMut;

use super::parser;

pub type DecodeResult = Result<Option<(usize, RespValue)>>;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum RespValue {
    SimpleString(String),
    Error(String),
    Integer(i64),
    Array(Vec<RespValue>),
    BulkString(Vec<u8>),
    Nil,
}

impl RespValue {
    pub fn from_bytes(input: &BytesMut) -> DecodeResult {
        parser::parse(input, 0)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            RespValue::SimpleString(s) => s.clone().into_bytes(),
            RespValue::Error(s) => s.clone().into_bytes(),
            RespValue::Integer(i) => i.to_string().into_bytes(),
            RespValue::BulkString(s) => s.to_vec(),
            RespValue::Array(_) => unimplemented!(),
            RespValue::Nil => unimplemented!(),
        }
    }
}
