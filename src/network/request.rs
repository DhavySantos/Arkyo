use super::Method;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Request {
    headers: HashMap<String, String>,
    params: HashMap<String, String>,
    protocol: String,
    method: Method,
    body: String,
    path: String,
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

    pub fn from_str(input: &str) -> Result<Self, ()> {
        let mut request = Self::new();

        let entry_regex =
            Regex::new(r"^(?P<method>\w+) (?P<path>[^\s]+) (?P<protocol>[^\s]+)").unwrap();
        let header_regex = Regex::new(r"(?i)^([^:\s]+):\s*(.*)$").unwrap();
        let body_regex = Regex::new(r"\r\n\r\n(.*)$").unwrap();

        let mut lines = input.lines();

        let entry = match lines.next() {
            Some(entry) => entry,
            None => return Err(()),
        };

        if let Some(captures) = entry_regex.captures(entry) {
            request.protocol = match captures.name("protocol") {
                Some(protocol) => String::from(protocol.as_str()),
                None => return Err(()),
            };

            request.method = match captures.name("method") {
                Some(method) => Method::from_str(method.as_str())?,
                None => return Err(()),
            };

            request.path = match captures.name("path") {
                Some(path) => String::from(path.as_str()),
                None => return Err(()),
            };
        };

        for line in input.lines() {
            if line.trim().is_empty() {
                break;
            };

            if let Some(captures) = header_regex.captures(line) {
                let key = String::from(captures.get(1).unwrap().as_str());
                let value = String::from(captures.get(2).unwrap().as_str());
                request.headers.insert(key, value);
            };
        }

        if let Some(captures) = body_regex.captures(input) {
            request.body = match captures.get(1) {
                Some(body) => String::from(body.as_str()),
                None => return Err(()),
            };
        };

        Ok(request)
    }
}
