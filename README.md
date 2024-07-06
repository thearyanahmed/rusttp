# RustTP

RustTP is a simple, lightweight HTTP router built in Rust. It uses the Tokio library for asynchronous I/O and can handle multiple concurrent connections.
This is a learning project to understand how HTTP servers work and how to build one from scratch. It is not intended for production use.

It is 

## Features

- Supports HTTP methods: GET, POST, PUT, DELETE, PATCH, OPTIONS, HEAD
- Route handling with custom logic
- Query parameter parsing
- Header parsing
- Error handling for invalid requests

## Usage

To use RustTP, you need to create a `Router` instance, add routes to it, and then start the server.

Here's a basic example:

```rust
use rusttp::{Method, Request, Response, Router};
use std::sync::Arc;
use std::{fs, io};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut router = Router::new();
    router.add_route(Method::GET, "/whoami", who_handler);
    router.add_route(Method::GET, "/page", page_handler);
    router.add_route(Method::POST, "/say-hi", say_hi_handler);

    let router = Arc::new(router);

    router
        .listen_and_serve("localhost:8000")
        .await
        .expect("failed to listen and serve");

    Ok(())
}
```

In this example, we're adding several routes to the router. Each route is associated with a handler function that takes a `Request` and returns a `Response`.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

RustTP is licensed under the MIT license. Please see the `LICENSE` file for more details.