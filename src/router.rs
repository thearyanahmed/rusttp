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
    pub headers: HashMap<String, String>,
    pub body: String,
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

        let request = self.parse_request(&buffer[..])?;

        let Request { method, path, .. } = self.parse_request(&buffer[..])?;

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

            stream.write_all(response_str.as_bytes()).await?;
            stream.flush().await?;
        }
        Ok(())
    }

    fn parse_request(&self, buffer: &[u8]) -> io::Result<Request> {
        let request_string = String::from_utf8_lossy(buffer);
        let mut lines = request_string.lines();

        // Parse request line
        let request_line = lines
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing request line"))?;
        let mut parts = request_line.split_whitespace();

        let method = self.parse_method(parts.next())?;
        let path = self.parse_path(parts.next())?;

        // Parse headers
        let mut headers = HashMap::new();
        for line in lines.by_ref() {
            if line.is_empty() {
                break; // End of headers
            }
            let mut header_parts = line.splitn(2, ':');
            if let Some(key) = header_parts.next() {
                let key = key.trim().to_string();
                let value = header_parts.next().unwrap_or("").trim().to_string();
                headers.insert(key, value);
            }
        }

        // Parse body
        let body = lines.collect::<Vec<&str>>().join("\n");

        Ok(Request {
            method,
            path,
            headers,
            body,
        })
    }

    fn parse_method(&self, method_str: Option<&str>) -> io::Result<Method> {
        match method_str {
            Some("GET") => Ok(Method::GET),
            Some("POST") => Ok(Method::POST),
            Some("PUT") => Ok(Method::PUT),
            Some("DELETE") => Ok(Method::DELETE),
            Some("PATCH") => Ok(Method::PATCH),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Unsupported method",
            )),
        }
    }

    fn parse_path(&self, path_str: Option<&str>) -> io::Result<String> {
        let mut path = path_str
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing path"))?
            .to_string();
        if let Some(pos) = path.find(['?', '#'].as_ref()) {
            path.truncate(pos);
        }
        Ok(path)
    }
}
