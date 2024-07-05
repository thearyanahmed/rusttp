use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use std::thread;
use std::time::Duration;

fn main() {
    println!("Journey of a thousand miles begins with a single commit.");

    let addr = "localhost:8000";

    let listener = TcpListener::bind(addr).expect("failed to bind to address localhost:8000");

    for incoming_stream in listener.incoming() {
        match incoming_stream {
            Ok(valid_stream) => handle_incoming_stream(valid_stream),
            Err(err) => eprint!("connection failed \nerr::{}", err),
        }
    }
}

fn handle_incoming_stream(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    println!("received request \n{}", request);

    let response = "Hello from server!";
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}\n\n",
        response.len(),
        response
    );

    thread::sleep(Duration::from_secs(10));

    stream
        .write_all(&response.as_bytes())
        .expect("failed to write to stream");
    stream.flush().expect("failed to flush response")
}
