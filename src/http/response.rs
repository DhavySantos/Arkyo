use crate::http::Status;
use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Response {
    headers: HashMap<String, String>,
    stream: TcpStream,
    version: String,
    status: Status,
    body: String,
}

impl Response {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            version: String::from("HTTP/1.1"),
            headers: HashMap::new(),
            body: String::new(),
            status: Status::Ok,
            stream,
        }
    }

    pub fn get_header<T: Into<String>>(&self, name: T) -> Option<&String> {
        self.headers.get(&name.into())
    }

    pub fn set_header<T: Into<String>>(&mut self, name: T, value: T) {
        self.headers.insert(name.into(), value.into());
    }

    pub fn body(&mut self, body: impl Into<String>) {
        self.body = body.into();
    }

    pub fn send(&mut self) {
        let bytes = self.as_bytes();
        self.stream.write_all(&bytes);
    }

    pub fn as_bytes(&mut self) -> Vec<u8> {
        let mut response_string = format!("{} 200 OK\r\n", self.version);

        if !self.body.is_empty() {
            let size = self.body.len().to_string();
            self.headers.insert(String::from("Content-Length"), size);
        }

        for (key, value) in &self.headers {
            response_string.push_str(&format!("{}: {}\r\n", key, value));
        }

        response_string.push_str("\r\n");
        response_string.push_str(&self.body);

        response_string.into_bytes()
    }
}
