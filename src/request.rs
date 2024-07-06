use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
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
    http_version: String,
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

    pub fn get_http_version(&self) -> String {
        self.http_version.clone()
    }
}

// Request parsing
impl Request {
    fn parse_path_and_query_params(path_with_query: &str) -> Option<(String, HashMap<String, String>)> {
        // Replace with your actual logic to parse path and query params
        // For demonstration, split path and query params by "?" and parse query params
        let parts: Vec<&str> = path_with_query.splitn(2, '?').collect();
        let path = parts[0].to_string();
        
        if parts.len() > 1 {
            let query_params_str = parts[1];
            let query_params: HashMap<String, String> = query_params_str
                .split('&')
                .filter_map(|param| {
                    let mut parts = param.split('=');
                    let key = parts.next()?.to_string();
                    let value = parts.next()?.to_string();
                    Some((key, value))
                })
                .collect();
            Some((path, query_params))
        } else {
            Some((path, HashMap::new()))
        }
    }

    fn parse_request_line(request_line: &str) -> Result<(Method, String, HashMap<String,String>, String), std::io::Error> {
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        if parts.len() != 3 {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid request line"));
        }

        let method_str = parts[0];
        let path_with_query = parts[1];
        let http_version = parts[2];

        let method = Request::parse_method(Some(method_str))?;

        let (path, query_params) = Request::parse_path_and_query_params(path_with_query)
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid path"))?;

        Ok((method, path, query_params, http_version.to_string()))
    }

    pub fn from_parts(request_line: &str, headers_and_body: &[&str]) -> Result<Self, std::io::Error> {
        let (method, path, query_params, http_version) = Request::parse_request_line(request_line)?;
        
        let mut headers = HashMap::new();
        let mut body = String::new();
        let mut headers_and_body_iter = headers_and_body.iter().copied();

        // Parse headers
        while let Some(header_line) = headers_and_body_iter.next() {
            if header_line.is_empty() {
                // Reached end of headers, rest is body
                break;
            }
            if let Some((key, value)) = header_line.split_once(": ") {
                headers.insert(key.to_string(), value.to_string());
            }
        }

        // Parse body
        for line in headers_and_body_iter {
            body.push_str(line);
            body.push('\n'); // Assuming body lines are separated by newline
        }

        Ok(Request {
            method,
            path,
            headers,
            body,
            query_params,
            http_version,
        })
    }

    pub fn from_u8_buffer(buffer: &[u8]) -> io::Result<Request> {
        let mut request_string = String::from_utf8_lossy(buffer).into_owned();

        if let Some(null_index) = request_string.find('\0') {
            request_string.truncate(null_index);
        }

        let parts : Vec<_> = request_string.split("\r\n").collect();

        if parts.len() == 0 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "missing request line, length is zero"));
        }
        
        if let Some((request_line, rest)) = parts.split_first() {
            let request = Request::from_parts(request_line, rest)?;
            return Ok(request);
        }

        Err(io::Error::new(io::ErrorKind::InvalidData, "missing request line"))
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

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_path_and_query_params_parses_path_without_query_params_correctly() {
        let (path, query_params) = Request::parse_path_and_query_params("/path").unwrap();
        assert_eq!(path, "/path");
        assert!(query_params.is_empty());
    }

    #[test]
    fn parse_path_and_query_params_parses_path_with_query_params_correctly() {
        let (path, query_params) = Request::parse_path_and_query_params("/path?key=value").unwrap();
        assert_eq!(path, "/path");
        assert_eq!(query_params.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn parse_request_line_parses_valid_request_line() {
        let result = Request::parse_request_line("GET /path HTTP/1.1").unwrap();
        assert_eq!(result.0, Method::GET);
        assert_eq!(result.1, "/path");
        assert!(result.2.is_empty());
        assert_eq!(result.3, "HTTP/1.1");
    }

    #[test]
    fn parse_request_line_returns_error_for_invalid_request_line() {
        let result = Request::parse_request_line("INVALID /path HTTP/1.1");
        assert!(result.is_err());
    }

    #[test]
    fn from_parts_parses_valid_parts() {
        let result = Request::from_parts("GET /path HTTP/1.1", &["Host: example.com", ""]).unwrap();
        assert_eq!(result.get_method(), Method::GET);
        assert_eq!(result.get_path(), "/path");
        assert_eq!(result.get_header("Host".to_string()), Some(&"example.com".to_string()));
    }

    #[test]
    fn from_parts_returns_error_for_invalid_parts() {
        let result = Request::from_parts("INVALID /path HTTP/1.1", &["Host: example.com", ""]);
        assert!(result.is_err());
    }

    #[test]
    fn from_u8_buffer_parses_valid_request() {
        let request = b"GET /path?key=hello HTTP/1.1\r\nHost: example.com\r\n\r\n";
        let result = Request::from_u8_buffer(request).unwrap();
        assert_eq!(result.get_method(), Method::GET);
        assert_eq!(result.get_path(), "/path");
        assert_eq!(result.get_header("Host".to_string()), Some(&"example.com".to_string()));
        assert_eq!(result.get_query_param("key"), Some(&"hello".to_string()));
    }

    #[test]
    fn from_u8_buffer_returns_error_for_invalid_request() {
        let request = b"INVALID /path HTTP/1.1\r\nHost: example.com\r\n\r\n";
        let result = Request::from_u8_buffer(request);
        assert!(result.is_err());
    }

    #[test]
    fn parse_method_parses_valid_method() {
        let result = Request::parse_method(Some("GET")).unwrap();
        assert_eq!(result, Method::GET);
    }

    #[test]
    fn parse_method_returns_error_for_invalid_method() {
        let result = Request::parse_method(Some("INVALID"));
        assert!(result.is_err());
    }
}