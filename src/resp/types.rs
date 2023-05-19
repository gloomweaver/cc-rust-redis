use super::parser;

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
    pub fn from_bytes(input: &[u8]) -> Option<RespValue> {
        parser::parse(input)
    }
}
