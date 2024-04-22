use super::Method;
use lazy_static::lazy_static;
use regex::Error as RError;
use regex::{Captures, Regex};
use std::collections::HashMap;
use std::str::FromStr;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum RequestErrors {
    Regex(RError),
    Entry(String),
    NotHTTP,
    InvalidMethod,
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Request {
    headers: HashMap<String, String>,
    params: HashMap<String, String>,
    protocol: String,
    method: Method,
    body: String,
    path: String,
}

static HEADER_REGEX_STR: &str = r"(?P<key>[^:\s]+): *(?P<value>.*)";
lazy_static! {
    static ref HTTP_REGEX: Regex = Regex::new(format!(r"^(?P<method>\w+) (?P<path>[^\s]+) (?P<version>[^\s]+)(?P<headers>(\n{HEADER_REGEX_STR})*)(\n\n(?P<body>(?s:.*)))\z").as_str()).unwrap();
    static ref HEADER_REGEX: Regex = Regex::new(format!("({HEADER_REGEX_STR})").as_str()).unwrap();
}

impl Request {
    fn new() -> Self {
        Self {
            protocol: String::new(),
            headers: HashMap::new(),
            params: HashMap::new(),
            method: Method::Get,
            body: String::new(),
            path: String::new(),
        }
    }

    pub fn set_params(&mut self, params: HashMap<String, String>) {
        self.params = params;
    }

    #[must_use]
    pub const fn params(&self) -> &HashMap<String, String> {
        &self.params
    }

    #[must_use]
    pub const fn path(&self) -> &String {
        &self.path
    }

    #[must_use]
    pub const fn method(&self) -> &Method {
        &self.method
    }

    fn parse_http(content: Captures) -> Result<Request, RequestErrors> {
        let mut request = Request::new();

        request.method = match Method::from_str(
            content
                .name("method")
                .expect("Method must be provided in http.")
                .as_str(),
        ) {
            Ok(method) => method,
            Err(_) => return Err(RequestErrors::InvalidMethod),
        };

        request.path = String::from(
            content
                .name("path")
                .expect("Path must be provided in http.")
                .as_str(),
        );

        request.protocol = String::from(
            content
                .name("version")
                .expect("Protocol version must be provided in http.")
                .as_str(),
        );

        request.body = String::from(content.name("body").map_or("", |value| value.as_str()));

        let header_content = content.name("headers").map_or("", |value| value.as_str());
        request.headers = HEADER_REGEX
            .captures_iter(header_content)
            .map(|header| {
                (
                    String::from(header.name("key").map_or("", |value| value.as_str())),
                    String::from(header.name("value").map_or("", |value| value.as_str())),
                )
            })
            .collect();

        Ok(request)
    }
}

impl FromStr for Request {
    type Err = RequestErrors;

    fn from_str(input: &str) -> Result<Request, Self::Err> {
        match HTTP_REGEX.captures(input) {
            None => Err(RequestErrors::NotHTTP),
            Some(content) => Self::parse_http(content),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regex_test() {
        let request = "GET /this/specific/path HTTP/1.1
Host: 127.0.0.1:8888
Connection: keep-alive
empty-header-test:
no-space:test

body content";

        let result = Request::from_str(request);
        println!("{:?}", result);
    }
}
