use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    println!("Journey of a thousand miles begins with a single commit.");

    let addr = "localhost:8000";

    let listener = TcpListener::bind(addr).expect("failed to bind to address localhost:8000");

    // map("get","/route","handler(request) -> Response { status: 200, content: '' }")

    for incoming_stream in listener.incoming() {
        match incoming_stream {
            Ok(valid_stream) => {
                std::thread::spawn(|| handle_incoming_stream(valid_stream));
            }
            Err(err) => eprint!("connection failed \nerr::{}", err),
        }
    }
}

struct Request {
    path: String,
}

struct Response {
    status: u8,
}

struct Router {
    routes: Vec<(String, String, fn(Request) -> Response)>,
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

    stream
        .write_all(&response.as_bytes())
        .expect("failed to write to stream");
    stream.flush().expect("failed to flush response")
}
