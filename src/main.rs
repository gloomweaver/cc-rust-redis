use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;
use bytes::{BufMut, BytesMut};
use redis_starter_rust::{command::Command, resp};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<()> {
    let cache = Arc::new(Mutex::new(HashMap::new()));
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let incoming = listener.accept().await;
        match incoming {
            Ok((stream, _)) => {
                println!("new connection");
                let memory = cache.clone();

                tokio::spawn(async move {
                    handle_connection(stream, memory).await.unwrap();
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

async fn handle_connection(
    mut stream: TcpStream,
    memory: Arc<Mutex<HashMap<String, String>>>,
) -> Result<()> {
    let mut buf = BytesMut::with_capacity(512);
    loop {
        let bytes_read = stream.read_buf(&mut buf).await?;
        if bytes_read == 0 {
            println!("client closed the connection");
            break;
        }
        if let Some((_, resp)) = resp::RespValue::from_bytes(&buf)? {
            if let Some(command) = Command::from_resp_value(&resp) {
                match command {
                    Command::Ping(arg) => {
                        let mut out = BytesMut::with_capacity(512);
                        out.put_slice(b"+");
                        out.put_slice(&arg.as_bytes());
                        out.put_slice(b"\r\n");

                        stream.write_all(&out).await?;
                    }
                    Command::Echo(arg) => {
                        let mut out = BytesMut::with_capacity(512);
                        out.put_slice(b"+");
                        out.put_slice(&arg.as_bytes());
                        out.put_slice(b"\r\n");

                        stream.write_all(&out).await?;
                    }
                    Command::Get(key) => {
                        let mut out = BytesMut::with_capacity(512);
                        let mem = memory.lock().await;
                        let val = mem.get(&key).unwrap();

                        out.put_slice(b"+");
                        out.put_slice(&val.as_bytes());
                        out.put_slice(b"\r\n");

                        stream.write_all(&out).await?;
                    }
                    Command::Set(key, value) => {
                        let mut mem = memory.lock().await;
                        mem.insert(key, value);
                        stream.write_all(b"+OK\r\n").await?;
                    }
                }
            } else {
                stream.write_all(b"-ERR unknown command\r\n").await?;
            }
            buf.clear();
        }
    }
    Ok(())
}
