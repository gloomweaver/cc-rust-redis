use anyhow::Result;
use bytes::{BufMut, BytesMut};
use redis_starter_rust::{command::Command, resp};
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
                }
            } else {
                stream.write_all(b"-ERR unknown command\r\n").await?;
            }
            buf.clear();
        }
    }
    Ok(())
}
