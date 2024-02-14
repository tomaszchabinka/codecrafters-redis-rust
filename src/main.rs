use std::error::Error;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn handle_connection(mut _stream: TcpStream) -> Result<(), Box<dyn Error>> {
    println!("handle_connection");
    let mut buf = vec![0; 8196];
    while let Ok(n) = _stream.read(&mut buf) {
        if n == 0 {
            break;
        }
        //sleep(Duration::from_millis(5 * 1000));
        println!("received {} bytes", n);
        println!("data: {:?}", String::from_utf8(Vec::from(&buf[0..n]))?);
        _ = _stream.write(b"+PONG\r\n");
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => _ = handle_connection(_stream),
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}
