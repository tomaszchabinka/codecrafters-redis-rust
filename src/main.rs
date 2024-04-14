use std::{
    error::Error,
    io::{Read, Write},
};
use tokio::{
    io::{self},
    net::{TcpListener, TcpStream},
};

async fn handle_connection(stream: TcpStream) {
    println!("handle_connection");
    let mut buf = vec![0; 8196];

    let mut std_tcp_stream = stream.into_std().unwrap();
    std_tcp_stream.set_nonblocking(false).unwrap();

    loop {
        match std_tcp_stream.read(&mut buf) {
            Ok(0) => {
                println!("empty input");
                break;
            }
            Ok(n) => {
                println!("read {} bytes", n);
                println!(
                    "data: {:?}",
                    String::from_utf8(Vec::from(&buf[0..n])).unwrap()
                );
                _ = std_tcp_stream.write_all(b"+PONG\r\n");
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                println!("io::ErrorKind::WouldBlock");
                continue;
            }
            Err(_e) => {
                println!("break");
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        match listener.accept().await {
            Ok((mut _stream, _addr)) => {
                println!("new connection {:?}", _addr);
                tokio::spawn(async move { handle_connection(_stream).await });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
