use anyhow::Result;
use bytes::BytesMut;
use redis_starter_rust::resp;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<()> {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let incoming = listener.accept().await;
        match incoming {
            Ok((stream, _)) => {
                println!("new connection");

                tokio::spawn(async move {
                    handle_connection(stream).await.unwrap();
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

async fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let mut buf = BytesMut::with_capacity(512);
    loop {
        // Wait for the client to send us a message but ignore the content for now
        let bytes_read = stream.read_buf(&mut buf).await?;
        if bytes_read == 0 {
            println!("client closed the connection");
            break;
        }

        let command = resp::RespValue::from_bytes(&buf[..bytes_read]);
        match command {
            Some(command) => match command {
                resp::RespValue::Array(array) => {
                    stream.write_all(b"").await?;
                    match array.get(0) {
                        Some(resp::RespValue::SimpleString(command)) => {
                            if command == "ping" {
                                stream.write_all(b"+PONG\r\n").await?;
                            } else if command == "echo" {
                                if let Some(arg) = array.get(1) {
                                    if let resp::RespValue::SimpleString(arg) = arg {
                                        stream.write_all(b"+").await?;
                                        stream.write_all(arg.as_bytes()).await?;
                                        stream.write_all(b"\r\n").await?;
                                    }
                                } else {
                                    stream.write_all(b"-ERR unknown command\r\n").await?;
                                }
                            } else {
                                stream.write_all(b"-ERR unknown command\r\n").await?;
                            }
                        }
                        _ => {
                            stream.write_all(b"-ERR unknown command\r\n").await?;
                        }
                    }
                }
                _ => {
                    println!("not an array");
                    stream.write_all(b"-ERR unknown command\r\n").await?;
                }
            },
            None => {
                stream.write_all(b"-ERR unknown command\r\n").await?;
            }
        }
        buf.clear();
    }
    Ok(())
}
