use std::collections::HashMap;
use std::fmt;
use std::io;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    OPTIONS,
    HEAD,
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let method_str = match *self {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::DELETE => "DELETE",
            Method::PATCH => "PATCH",
            Method::OPTIONS => "OPTIONS",
            Method::HEAD => "HEAD",
        };
        write!(f, "{}", method_str)
    }
}

pub struct Request {
    method: Method,
    path: String,
    headers: HashMap<String, String>,
    body: String,
    query_params: HashMap<String, String>,
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

    pub fn get_body(&self) -> String {
        self.body.clone()
    }

    pub fn get_query_param(&self, key: &str) -> Option<&String> {
        self.query_params.get(key)
    }
}

// Request parsing
impl Request {
    pub fn from_u8_buffer(buffer: &[u8]) -> io::Result<Request> {
        let mut request_string = String::from_utf8_lossy(buffer).into_owned();

        let filters: [&str; 2] = ["\r\n\r\n", "\0\0\0"];

        for filter in filters {
            if let Some(index) = request_string.find(filter) {
                request_string.truncate(index);
            }
        }

        let mut lines = request_string.lines();

        // Parse request line
        let request_line = lines
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing request line"))?;
        let mut parts: std::str::SplitWhitespace = request_line.split_whitespace();

        let method = Request::parse_method(parts.next())?;
        let path_with_query = parts.next().unwrap_or("");
        let (path, query_params) = Request::parse_path_and_query_params(path_with_query)?;

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
            query_params,
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
                "Unsupported method!",
            )),
        }
    }

    fn parse_path_and_query_params(
        path_with_query: &str,
    ) -> io::Result<(String, HashMap<String, String>)> {
        let mut parts = path_with_query.splitn(2, '?');
        let path = parts.next().unwrap_or("").to_string();
        let query_params = if let Some(query_part) = parts.next() {
            Request::parse_query_params(query_part)
        } else {
            HashMap::new()
        };
        Ok((path, query_params))
    }

    fn parse_query_params(query_part: &str) -> HashMap<String, String> {
        let mut query_params = HashMap::new();
        for pair in query_part.split('&') {
            if let Some(pos) = pair.find('=') {
                let key = &pair[..pos];
                let value = &pair[pos + 1..];
                query_params.insert(key.to_string(), value.to_string());
            }
        }
        query_params
    }
}
