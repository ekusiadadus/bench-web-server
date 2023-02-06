// use std::io::{Read, Write};
// use std::net::{TcpListener, TcpStream};
// use std::thread;

// fn main() {
//     let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
//     for stream in listener.incoming() {
//         match stream {
//             Ok(stream) => {
//                 thread::spawn(move || handle_client(stream));
//             }
//             Err(e) => {
//                 println!("error");
//             }
//         }
//     }
// }

// fn handle_client(mut stream: TcpStream) {
//     let mut buf = [0; 512];
//     match stream.read(&mut buf) {
//         Ok(_) => {
//             let response = "HTTP/1.1 200 Ok\r\nContent-Type: text/html\r\n\r\n<h1>Hello World</h1>";
//             stream.write(response.as_bytes());
//         }
//         Err(e) => {
//             println!("error")
//         }
//     }
// }

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