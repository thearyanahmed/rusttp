use std::collections::HashMap;
use std::io;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

pub struct Request {
    method: Method,
    path: String,
    headers: HashMap<String, String>,
    body: String,
}

impl Request {
    pub fn get_header(&self, key: String) -> Option<&String> {
        self.headers.get(&key)
    }

    pub fn get_method(&self) -> Method {
        self.method.clone()
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }
}

// Request parsing
impl Request {
    pub fn from_u8_buffer(buffer: &[u8]) -> io::Result<Request> {
        let request_string = String::from_utf8_lossy(buffer);

        let mut lines = request_string.lines();

        // Parse request line
        let request_line = lines
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing request line"))?;
        let mut parts = request_line.split_whitespace();

        let method = Request::parse_method(parts.next())?;
        let path = Request::parse_path(parts.next())?;

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

    fn parse_method(method_str: Option<&str>) -> io::Result<Method> {
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

    fn parse_path(path_str: Option<&str>) -> io::Result<String> {
        let mut path = path_str
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing path"))?
            .to_string();
        if let Some(pos) = path.find(['?', '#'].as_ref()) {
            path.truncate(pos);
        }
        Ok(path)
    }
}
