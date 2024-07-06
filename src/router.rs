use std::collections::HashMap;
use std::io;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub struct Router {
    routes: HashMap<(Method, String), Box<dyn Fn(Request) -> Response + Send + Sync>>,
}

pub struct Request {
    pub method: Method,
    pub path: String,
}

pub struct Response {
    pub status: u8,
    pub content: String,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn add_route<F>(&mut self, method: Method, path: &str, handler: F)
    where
        F: Fn(Request) -> Response + 'static + std::marker::Sync + std::marker::Send,
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

        let request_str = String::from_utf8_lossy(&buffer[..]);
        println!("received request \n{}", request_str);

        // let request = parse_request(&request_str)?;
        let request = parse_request(&request_str)?;

        let Request { method, path, .. } = parse_request(&request_str)?;

        if let Some(handler) = self.routes.get(&(method, path.clone())) {
            let response = handler(request);

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

            // let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
            stream.write_all(response_str.as_bytes()).await?;
            stream.flush().await?;
        }
        Ok(())
    }
}

// Placeholder for request parsing logic
fn parse_request(request_str: &str) -> Result<Request, io::Error> {
    // Implement request parsing logic here based on your actual needs
    // For simplicity, this example assumes basic parsing to extract method and path
    let parts: Vec<&str> = request_str
        .lines()
        .next()
        .unwrap_or("")
        .split(' ')
        .collect();

    if parts.len() < 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid request format",
        ));
    }

    let method = match parts[0] {
        "GET" => Method::GET,
        "POST" => Method::POST,
        "PUT" => Method::PUT,
        "DELETE" => Method::DELETE,
        "PATCH" => Method::PATCH,
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Unsupported method",
            ))
        }
    };

    let mut path = parts[1].to_string();
    if let Some(pos) = path.find(['?', '#'].as_ref()) {
        path.truncate(pos);
    }

    Ok(Request { method, path })
}
