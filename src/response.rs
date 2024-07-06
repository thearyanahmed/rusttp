use std::collections::HashMap;

pub struct Response {
    pub status: u16,
    pub content: String,
    pub headers: HashMap<String, String>,
}

impl Response {
    pub fn new() -> Self {
        Response {
            status: 0,
            content: String::new(),
            headers: HashMap::new(),
        }
    }

    pub fn success() -> Self {
        let mut response = Response::new();
        response.set_status(200);

        response
    }

    pub fn set_status(&mut self, status: u16) {
        self.status = status;
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }

    pub fn set_headers(&mut self, headers: HashMap<String, String>) {
        self.headers = headers;
    }

    pub fn set_header(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }
}

impl Response {
    pub fn build_http_response(&self) -> String {
        // Get the status text for the status code
        let status_text = self.status_text();

        // Build the response string with headers and content
        let mut response_str = format!("HTTP/1.1 {} {}\r\n", self.status, status_text);

        for (key, value) in &self.headers {
            response_str.push_str(&format!("{}: {}\r\n", key, value));
        }

        response_str.push_str(&format!(
            "Content-Length: {}\r\n\r\n{}\n\n",
            self.content.len(),
            self.content
        ));

        response_str
    }

    pub fn default_response() -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/plain".to_string());

        Self {
            status: 404,
            headers,
            content: "404 - Not Found".to_string(),
        }
    }

    fn status_text(&self) -> &'static str {
        match self.status {
            100 => "Continue",
            101 => "Switching Protocols",
            102 => "Processing",
            103 => "Early Hints",
            200 => "OK",
            201 => "Created",
            202 => "Accepted",
            203 => "Non-Authoritative Information",
            204 => "No Content",
            205 => "Reset Content",
            206 => "Partial Content",
            207 => "Multi-Status",
            208 => "Already Reported",
            226 => "IM Used",
            300 => "Multiple Choices",
            301 => "Moved Permanently",
            302 => "Found",
            303 => "See Other",
            304 => "Not Modified",
            305 => "Use Proxy",
            307 => "Temporary Redirect",
            308 => "Permanent Redirect",
            400 => "Bad Request",
            401 => "Unauthorized",
            402 => "Payment Required",
            403 => "Forbidden",
            404 => "Not Found",
            405 => "Method Not Allowed",
            406 => "Not Acceptable",
            407 => "Proxy Authentication Required",
            408 => "Request Timeout",
            409 => "Conflict",
            410 => "Gone",
            411 => "Length Required",
            412 => "Precondition Failed",
            413 => "Payload Too Large",
            414 => "URI Too Long",
            415 => "Unsupported Media Type",
            416 => "Range Not Satisfiable",
            417 => "Expectation Failed",
            418 => "I'm a Teapot",
            421 => "Misdirected Request",
            422 => "Unprocessable Entity",
            423 => "Locked",
            424 => "Failed Dependency",
            425 => "Too Early",
            426 => "Upgrade Required",
            428 => "Precondition Required",
            429 => "Too Many Requests",
            431 => "Request Header Fields Too Large",
            451 => "Unavailable For Legal Reasons",
            500 => "Internal Server Error",
            501 => "Not Implemented",
            502 => "Bad Gateway",
            503 => "Service Unavailable",
            504 => "Gateway Timeout",
            505 => "HTTP Version Not Supported",
            506 => "Variant Also Negotiates",
            507 => "Insufficient Storage",
            508 => "Loop Detected",
            510 => "Not Extended",
            511 => "Network Authentication Required",
            _ => "Unknown Status",
        }
    }
}
