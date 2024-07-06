use crate::request::Method;
use crate::Request;
use std::collections::HashMap;
use std::io;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub struct Router {
    routes: HashMap<(Method, String), Box<dyn Fn(&Request) -> Response + Send + Sync>>,
}

pub struct Response {
    pub status: u8,
    pub content: String,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn add_route<F>(&mut self, method: Method, path: &str, handler: F)
    where
        F: Fn(&Request) -> Response + 'static + std::marker::Sync + std::marker::Send,
    {
        self.routes
            .insert((method, path.to_string()), Box::new(handler));
    }

    pub async fn listen_and_serve(self: Arc<Self>, addr: &str) -> io::Result<()> {
        let listener = TcpListener::bind(addr).await?;

        loop {
            let (socket, _) = listener.accept().await?;
            let router_clone: Arc<Router> = Arc::clone(&self); // Clone Arc<Self> for each task

            tokio::spawn(async move {
                if let Err(e) = router_clone.handle_incoming_stream(socket).await {
                    eprintln!("failed to handle connection: {}", e);
                }
            });
        }
    }

    async fn handle_incoming_stream(&self, mut stream: TcpStream) -> io::Result<()> {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).await?;

        let request = Request::from_u8_buffer(&buffer[..])?;
        // let path = &request.get_path();

        if let Some(handler) = self
            .routes
            .get(&(request.get_method(), request.get_path().to_string()))
        {
            let response = handler(&request); // Pass request by reference

            let response_str = format!(
                "HTTP/1.1 {} OK\r\nContent-Length: {}\r\n\r\n{}\n\n",
                response.status,
                response.content.len(),
                response.content
            );
            stream.write_all(response_str.as_bytes()).await?;
            stream.flush().await?;
        } else {
            // Handle 404 Not Found
            let response_str = format!(
                "HTTP/1.1 {} OK\r\nContent-Length: {}\r\n\r\n{}\n\n",
                404,
                "Not Found".len(),
                "Not Found"
            );

            stream.write_all(response_str.as_bytes()).await?;
            stream.flush().await?;
        }
        Ok(())
    }
}
