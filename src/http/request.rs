use crate::util::Location;
use once_cell::sync::Lazy;
use regex::Regex;

use std::borrow::Cow;
use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

use crate::http::Method;

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Request {
    headers: HashMap<String, String>,
    params: HashMap<String, String>,
    pub(crate) location: String,
    pub(crate) version: String,
    method: Method,
    body: String,
}

const HTTP: &str = r#"(?P<method>\w+)\s+(?P<location>[^\s]+)\s+(?P<version>[^\s]+)"#;
const HEADER: &str = r"(?P<key>[^:\s]+):\s*(?P<value>.*)";
const BODY: &str = r#"(\n\n(?P<body>(?s:.*)))"#;

static R_HEADER: Lazy<Regex> = Lazy::new(|| Regex::new(HEADER).unwrap());
static R_HTTP: Lazy<Regex> = Lazy::new(|| Regex::new(HTTP).unwrap());
static R_BODY: Lazy<Regex> = Lazy::new(|| Regex::new(BODY).unwrap());

impl Request {
    pub fn from(text: Cow<'_, str>) -> Result<Self, Box<dyn Error>> {
        // Match the request line (method, location, version)
        if let Some(captures) = R_HTTP.captures(&text) {
            let location = String::from(captures.name("location").map_or("", |v| v.as_str()));
            let version = String::from(captures.name("version").map_or("", |v| v.as_str()));
            let method = String::from(captures.name("method").map_or("", |v| v.as_str()));

            let method = Method::from_str(&method)?;

            // Parse headers and body
            if let Some(pos) = text.find("\r\n\r\n") {
                let headers_section = &text[..pos];
                let body_section = &text[pos + 4..];

                let mut headers = HashMap::new();

                // Parse the headers using the HEADER regex
                for header in R_HEADER.captures_iter(headers_section) {
                    let key = String::from(header.name("key").map_or("", |v| v.as_str()));
                    let value = String::from(header.name("value").map_or("", |v| v.as_str()));
                    headers.insert(key, value);
                }

                // Body is the remaining part after the headers
                let body = String::from(body_section);
                let params = HashMap::new();

                return Ok(Self {
                    location,
                    version,
                    headers,
                    method,
                    params,
                    body,
                });
            }
        }

        Err("Invalid HTTP format".into())
    }

    pub fn fetch_params(&mut self, location: Location) {
        for capture in location.as_regex().captures_iter(&self.location) {}
    }

    pub fn method(&self) -> &Method {
        &self.method
    }
}
