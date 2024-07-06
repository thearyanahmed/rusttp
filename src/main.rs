use rustnet::Router;
#[allow(dead_code)]
use std::io;
use std::sync::Arc;

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
