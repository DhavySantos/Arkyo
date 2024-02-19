use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub struct Response {
    pub headers: Option<HashMap<String, String>>,
    pub status_code: Option<u16>,
    pub body: Option<String>,
}

impl ToString for Response {

    fn to_string(&self) -> String {
        let mut response = String::new();

        if let Some(status_code) = self.status_code {
            response.push_str(&format!("HTTP/1.1 {}\n", status_code));
        }

        if let Some(headers) = &self.headers {
            for (key, value) in headers {
                response.push_str(&format!("{}: {}\n", key, value));
            }
        }

        if let Some(body) = &self.body {
            response.push_str("\n");
            response.push_str(body);
        }

        response
    }
}

impl Response {
    pub fn new() -> Self {
        Self {
            status_code: None,
            headers: None,
            body: None,
        }
    }

    pub fn status (mut self, status_code: u16) -> Self {
        self.status_code = Some(status_code);
        self
    }

    pub fn body (mut self, body: String) -> Self {
        self.body = Some(body);
        self
    }

    pub fn headers (mut self, headers: HashMap<String, String>) -> Self {
        self.headers = Some(headers);
        self
    }
}