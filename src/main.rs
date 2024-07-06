#[allow(dead_code)]
use std::io::{self};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("Journey of a thousand miles begins with a single commit.");

    let addr = "localhost:8000";

    let listener = TcpListener::bind(addr).await?;

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handle_incoming_stream(socket).await {
                eprintln!("Failed to handle connection: {}", e);
            }
        });
    }
}

async fn handle_incoming_stream(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await?;

    let request = String::from_utf8_lossy(&buffer[..]);
    println!("received request \n{}", request);

    let response = "Hello from server!";
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}\n\n",
        response.len(),
        response
    );

    stream.write_all(response.as_bytes()).await?;
    stream.flush().await?;

    Ok(())
}
