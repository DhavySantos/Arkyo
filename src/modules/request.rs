use std::{collections::HashMap, str::FromStr};

use super::route::Method;

#[derive(Debug)]
pub struct Request {
    pub headers: HashMap<String, String>,
    pub params: HashMap<String, String>,
    pub path: Vec<String>,
    pub method: Method,
    pub body: String,
}

impl Request {
    pub fn new(mut request: &str) -> Request {
        let body = request.split("\r\n\r\n").nth(1).unwrap();
        request = request.split("\r\n\r\n").nth(0).unwrap();

        let method = request.split(" ").nth(0).unwrap();
        let path = request.split(" ").nth(1).unwrap();

        let lines: Vec<&str> = request.split("\r\n").skip(1).collect();
        
        let mut headers = HashMap::new();

        for line in lines {
            let mut parts = line.split(": ");
            let key = parts.next().unwrap();
            let value = parts.next().unwrap();
            headers.insert(key.to_string(), value.to_string());
        }
        
        Request {
            path: path.split("/").skip(1).map(|s| s.to_string()).collect(),
            method: Method::from_str(method).unwrap(),
            body: body.to_string(),
            params: HashMap::new(),
            headers,
        }

    }
}