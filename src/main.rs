use std::{io::Write, net::TcpListener};

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                stream.write(b"+PONG\r\n").expect("write failed");
                stream.flush().expect("flush failed");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
