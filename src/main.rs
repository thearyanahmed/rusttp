use rustnet::{Method, Request, Response, Router};
#[allow(dead_code)]
use std::io;
use std::sync::Arc;

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("Journey of a thousand miles begins with a single commit.");

    let mut router = Router::new();
    router.add_route(Method::GET, "/hello", handle_hello);
    router.add_route(Method::POST, "/hello", handle_hello);

    let router = Arc::new(router);

    router
        .listen_and_serve("localhost:8000")
        .await
        .expect("failed to listen and serve");

    Ok(())
}

fn handle_hello(_req: &Request) -> Response {
    Response {
        status: 200,
        content: "Hello, world!".to_string(),
    }
}
