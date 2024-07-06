use rustnet::{Method, Request, Response, Router};
use std::io;
use std::sync::Arc;

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("Journey of a thousand miles begins with a single commit.");

    let mut router = Router::new();
    router.add_route(Method::GET, "/hello", handle_hello);
    router.add_route(Method::POST, "/hello", handle_hello);
    router.add_route(Method::GET, "/hello/world", handle_hello);

    let router = Arc::new(router);

    router
        .listen_and_serve("localhost:8000")
        .await
        .expect("failed to listen and serve");

    Ok(())
}

fn handle_hello(req: &Request) -> Response {
    let mut response = Response::success();
    response.set_content(format!(
        "Hello from {} - {}",
        req.get_method(),
        req.get_path()
    ));

    response
}
