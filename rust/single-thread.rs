use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let response = "HTTP/1.1 200 OK\r\n\r\n<h1>Hello World</h1>";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
