use super::RespValue;

fn parse_simple_string(buf: &[u8]) -> Option<RespValue> {
    if let Some(parsed_value) = String::from_utf8(buf.to_vec()).ok() {
        return Some(RespValue::SimpleString(parsed_value));
    }

    None
}

fn parse_error(buf: &[u8]) -> Option<RespValue> {
    todo!()
}

fn parse_integer(buf: &[u8]) -> Option<RespValue> {
    Some(RespValue::Integer(0))
}

fn parse_bulk_string(buf: &[u8]) -> Option<RespValue> {
    todo!()
}

fn parse_array(buf: &[u8]) -> Option<RespValue> {
    let len_str_bytes = buf
        .iter()
        .take_while(|b| **b != b'\r')
        .map(|b| *b)
        .collect::<Vec<u8>>();
    let len = String::from_utf8(len_str_bytes)
        .ok()
        .map(|num| num.parse::<usize>().ok())
        .flatten();

    if let Some(len) = len {
        let buf = &buf[len..];
        let mut array = Vec::with_capacity(len);

        let some = buf
            .split(|x| *x == b'\n')
            .map(|x| String::from_utf8(x.to_vec()).ok())
            .filter(|x| {
                if let Some(x) = x {
                    if x == "\r" || x == "" {
                        return false;
                    }
                    if x.contains('$') {
                        return false;
                    };
                }
                true
            })
            .map(|x| {
                if let Some(x) = x {
                    return Some(x.replace("\r", ""));
                }
                None
            })
            .collect::<Vec<Option<String>>>();

        println!("{:?}", some);

        for i in some {
            if let Some(i) = i {
                array.push(RespValue::SimpleString(i));
            }
        }

        return Some(RespValue::Array(array));
    }

    None
}

pub fn parse(buf: &[u8]) -> Option<RespValue> {
    match buf.first() {
        Some(b'+') => parse_simple_string(&buf[1..]),
        Some(b'-') => parse_error(&buf[1..]),
        Some(b':') => parse_integer(&buf[1..]),
        Some(b'$') => parse_bulk_string(&buf[1..]),
        Some(b'*') => parse_array(&buf[1..]),
        _ => None,
    }
}
