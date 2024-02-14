use std::error::Error;
use std::io::prelude::*;
use std::net::TcpListener;

fn main() -> Result<(), Box<dyn Error>> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                let mut buf = [0; 1024];
                let n = _stream.read(&mut buf)?;
                println!("received {} bytes", n);
                println!("data: {:?}", String::from_utf8(Vec::from(&buf[0..n]))?);
                _stream.write_all(b"+PONG\r\n")?;
                _stream.flush()?;
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}
