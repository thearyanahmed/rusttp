use rusttp::{Method, Request, Response, Router};
use std::sync::Arc;
use std::{fs, io};

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("Journey of a thousand miles begins with a single commit.");

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

fn who_handler(_req: &Request) -> Response {
    let mut response = Response::success();
    response.set_content("Hi, I'm Aryan!".to_string());
    response
}

fn say_hi_handler(_req: &Request) -> Response {
    let mut response = Response::success();
    response.set_header("Content-Type".to_string(), "application/json".to_string());

    response.set_content(
        r#"{ "message": "Hi, So, this is supposed to be a post method!"}"#.to_string(),
    );

    response
}

fn page_handler(req: &Request) -> Response {
    let mut response = Response::success();
    let query_param = req.get_query_param("view");

    // If no page query parameter is provided, return a 404 with a message
    if query_param.is_none() {
        response = Response::default_response();
        response.set_content("Please specify a page using ?view=value".to_owned());
        return response;
    }

    // Extract the page value from the query parameter
    let page = query_param.unwrap();

    // Construct the file path based on the requested page
    let file_path = format!("public/{}.html", page);

    // Check if the file exists
    if let Ok(content) = fs::read_to_string(&file_path) {
        response.set_header("Content-Type".to_string(), "text/html".to_string());
        response.set_content(content);
    } else {
        // If file not found, return a 404 response
        response = Response::default_response();
    }

    response
}
