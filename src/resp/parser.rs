use super::types::DecodeResult;
use super::RespValue;
use anyhow::Error;
use bytes::BytesMut;

pub fn parse(buf: &BytesMut, idx: usize) -> DecodeResult {
    let length = buf.len();

    if idx >= length {
        return Ok(None);
    }

    match buf[idx] {
        b'+' => parse_string(buf, idx + 1),
        b'-' => parse_error(buf, idx + 1),
        b':' => parse_integer(buf, idx + 1),
        b'$' => parse_bulk_string(buf, idx + 1),
        b'*' => parse_array(buf, idx + 1),
        _ => Ok(None),
    }
}

fn parse_string(buf: &BytesMut, idx: usize) -> DecodeResult {
    let length = buf.len();
    let mut end = idx;

    if idx >= length {
        return Ok(None);
    }

    while let Some(val) = buf.get(end) {
        end += 1;
        if *val == b'\n' {
            break;
        }
    }

    Ok(Some((
        end,
        RespValue::SimpleString(String::from_utf8_lossy(&buf[idx..end - 2]).to_string()),
    )))
}

fn parse_error(buf: &BytesMut, idx: usize) -> DecodeResult {
    let length = buf.len();
    let mut end = idx;

    if idx >= length {
        return Ok(None);
    }

    while let Some(val) = buf.get(end) {
        end += 1;
        if *val == b'\n' {
            break;
        }
    }

    Ok(Some((
        end,
        RespValue::Error(String::from_utf8_lossy(&buf[idx..end - 2]).to_string()),
    )))
}

fn parse_integer(buf: &BytesMut, idx: usize) -> Result<Option<(usize, RespValue)>, Error> {
    // TODO: repeats many times, refactor this
    let length = buf.len();
    let mut end = idx;

    if idx >= length {
        return Ok(None);
    }

    while let Some(val) = buf.get(end) {
        end += 1;
        if *val == b'\n' {
            break;
        }
    }

    Ok(Some((
        end,
        RespValue::Integer(
            String::from_utf8_lossy(&buf[idx..end - 2])
                .to_string()
                .parse::<i64>()?,
        ),
    )))
}

fn parse_bulk_string(buf: &BytesMut, idx: usize) -> DecodeResult {
    let mut end = idx;

    // Don't like it, refactor this
    let str_len = parse_integer(buf, idx)?;
    if let Some((idx, _)) = str_len {
        end = idx;
    }

    let str = parse_string(buf, end)?;

    if let Some((end, RespValue::SimpleString(str))) = str {
        return Ok(Some((end, RespValue::BulkString(str.into_bytes()))));
    }

    Ok(None)
}

fn parse_array(buf: &BytesMut, idx: usize) -> DecodeResult {
    let mut end: usize;

    let str_len = parse_integer(buf, idx)?;

    if let Some((idx, RespValue::Integer(val))) = str_len {
        let mut res = Vec::with_capacity(val as usize);
        end = idx;

        for _ in 0..val {
            let str = parse(buf, end)?;

            if let Some((new_end, val)) = str {
                res.push(val);
                end = new_end;
            }
        }

        if res.len() == val as usize {
            return Ok(Some((end, RespValue::Array(res))));
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use bytes::{BufMut, BytesMut};

    use crate::resp::RespValue;

    use super::parse;

    #[test]
    fn parse_simple_string_test() {
        let mut buf = BytesMut::with_capacity(512);
        buf.put_slice(b"+OK\r\n");

        assert_eq!(
            parse(&buf, 0).unwrap(),
            Some((5, RespValue::SimpleString("OK".into())))
        );
    }

    #[test]
    fn parse_error_test() {
        let mut buf = BytesMut::with_capacity(512);
        buf.put_slice(b"-Error Message\r\n");

        assert_eq!(
            parse(&buf, 0).unwrap(),
            Some((16, RespValue::Error("Error Message".into())))
        )
    }

    #[test]
    fn parse_integer_test() {
        let mut buf = BytesMut::with_capacity(512);
        buf.put_slice(b":1000\r\n");
        assert_eq!(parse(&buf, 0).unwrap(), Some((7, RespValue::Integer(1000))));
        buf.clear();

        buf.put_slice(b":-1000\r\n");
        assert_eq!(
            parse(&buf, 0).unwrap(),
            Some((8, RespValue::Integer(-1000)))
        );
    }

    #[test]
    fn parse_bulk_string_test() {
        let mut buf = BytesMut::with_capacity(512);
        buf.put_slice(b"$6\r\nfoobar\r\n");
        assert_eq!(
            parse(&buf, 0).unwrap(),
            Some((12, RespValue::BulkString(b"foobar".to_vec())))
        );
    }

    #[test]
    fn parse_array_test() {
        let mut buf = BytesMut::with_capacity(512);
        buf.put_slice(b"*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n");
        assert_eq!(
            parse(&buf, 0).unwrap(),
            Some((
                22,
                RespValue::Array(vec![
                    RespValue::BulkString(b"foo".to_vec()),
                    RespValue::BulkString(b"bar".to_vec())
                ])
            ))
        );
    }
}
