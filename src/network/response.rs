use crate::network::Status;
use std::collections::HashMap;

pub struct Response {
    headers: HashMap<String, String>,
    status: Status,
    body: String,
}

impl Response {
    pub fn new() -> Response {
        Response {
            headers: HashMap::new(),
            body: String::new(),
            status: Status::Ok,
        }
    }

    pub fn headers(&mut self, input: HashMap<String, String>) {
        self.headers = input;
    }

    pub fn status(&mut self, input: Status) {
        self.status = input;
    }

    pub fn body(&mut self, input: String) {
        self.body = input;
    }
}

impl ToString for Response {
    fn to_string(&self) -> String {
        let mut output = String::new();

        output.push_str(&format!(
            "HTTP/1.1 {} {}\n",
            self.status as u16,
            self.status.to_string()
        ));
        output.push_str("Server: arkyo/0.0.7\n");

        for (key, value) in &self.headers {
            output.push_str(&format!("{}: {}\n", key, value));
        }

        output.push_str(&format!("\n{}", self.body));

        output
    }
}
