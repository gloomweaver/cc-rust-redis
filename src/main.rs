use std::{
    io::{Read, Write},
    net::TcpListener,
    str,
};

fn main() -> std::io::Result<()> {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        println!("new stream");
        match stream {
            Ok(mut stream) => {
                let mut payload = [0; 512];
                loop {
                    stream.read(&mut payload)?;
                    match str::from_utf8(&payload) {
                        Ok(data) => {
                            if data.contains("ping") {
                                stream.write(b"+PONG\r\n")?;
                            } else {
                                stream.write(b"-ERR unknown command\r\n")?;
                            }
                        }
                        Err(error) => println!("Invalid UTF-8 sequence: {}", error),
                    }
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}
