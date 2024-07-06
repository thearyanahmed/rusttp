#[allow(dead_code)]
use std::io::{self};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

struct Router {}

impl Router {
    // it is not public because it's in the same module for this example.
    fn new() -> Self {
        Self {}
    }

    async fn listen_and_serve(self: Arc<Self>, addr: &str) -> io::Result<()> {
        // let addr = "localhost:8000";

        let listener = TcpListener::bind(addr).await?;

        loop {
            let (socket, _) = listener.accept().await?;
            let router_clone = Arc::clone(&self); // Clone Arc<Self> for each task

            tokio::spawn(async move {
                if let Err(e) = router_clone.handle_incoming_stream(socket).await {
                    eprintln!("Failed to handle connection: {}", e);
                }
            });
        }
    }

    async fn handle_incoming_stream(&self, mut stream: TcpStream) -> io::Result<()> {
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
}

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("Journey of a thousand miles begins with a single commit.");

    let router = Arc::new(Router::new());

    router
        .listen_and_serve("localhost:8000")
        .await
        .expect("failed to listen and serve");

    Ok(())
}
