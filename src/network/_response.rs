use std::{collections::HashMap, io::Write, net::TcpStream};

use crate::network::Status;

pub struct Response<'life> { 
    headers: HashMap<String, String>,
    stream: &'life mut TcpStream,
    status: Status,
    body: String,
}

impl<'life> Response<'life> { 
    pub fn new(stream: &'life mut TcpStream) -> Response<'life> { 
        Response {
            headers: HashMap::new(),
            body: String::new(),
            status: Status::Ok,
            stream
        }
    }
    
    pub fn headers(&mut self, input: HashMap<String, String>) {
        self.headers = input;
    }
    
    pub fn status(&mut self, input: Status)  {
        self.status = input;
    }

    pub fn body(&mut self, input: String) {
        self.body = input;
    }

    pub fn send(&mut self) {
        let payload = self.to_string();
        self.stream.write_all(payload.as_bytes()).unwrap();
    }
}

impl<'life> ToString for Response<'life> { 
    
    fn to_string(&self) -> String {
        let mut output = String::new();

        output.push_str(&format!("HTTP/1.1 {} {}\n", self.status.code(), self.status.to_string()));
        output.push_str("Server: arkyo/0.0.6 (unix)\n");

        for (key, value) in &self.headers {
            output.push_str(&format!("{}: {}\n", key, value));
        }

        output.push_str(&format!("\n{}", self.body));

        output
    }

}
