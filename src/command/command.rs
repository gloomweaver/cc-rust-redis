use super::super::resp::RespValue;

pub enum Command {
    Ping(String),
    Echo(String),
}

impl Command {
    pub fn from_resp_value(resp_value: &RespValue) -> Option<Self> {
        match resp_value {
            RespValue::Array(values) => {
                let mut iter = values.iter();
                if let Some(RespValue::BulkString(command_name)) = iter.next() {
                    let command_name = String::from_utf8_lossy(command_name).to_string();

                    match command_name.as_str() {
                        // TODO: refactor this later
                        "ping" => {
                            let arg = iter.next().map(|arg| {
                                String::from_utf8_lossy(arg.as_bytes().clone().as_ref()).to_string()
                            });

                            if let Some(arg) = arg {
                                Some(Command::Ping(arg))
                            } else {
                                Some(Command::Ping("PONG".to_string()))
                            }
                        }
                        "echo" => {
                            let arg = iter
                                .next()
                                .map(|arg| {
                                    String::from_utf8_lossy(arg.as_bytes().clone().as_ref())
                                        .to_string()
                                })
                                .expect("WHAT");

                            Some(Command::Echo(arg))
                        }
                        _ => None,
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
