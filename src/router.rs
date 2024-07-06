use crate::request::Method;
use crate::{Request, Response};
use std::collections::HashMap;
use std::io;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub struct Router {
    routes: HashMap<(Method, String), Box<dyn Fn(&Request) -> Response + Send + Sync>>,
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
        let mut buffer = [0; 4096];
        stream.read(&mut buffer).await?;

        let request = Request::from_u8_buffer(&buffer[..])?;

        let response = match self
            .routes
            .get(&(request.get_method(), request.get_path().to_string()))
        {
            Some(handler) => handler(&request),
            None => Response::default_response(),
        };

        stream
            .write_all(response.build_http_response().as_bytes())
            .await?;
        stream.flush().await?;

        Ok(())
    }
}
