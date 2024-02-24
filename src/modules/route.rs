use std::str::FromStr;

use super::{request::Request, response::Response};

#[derive(Debug, Clone)]
pub struct Route {
    pub method: Method,
    pub path: Vec<String>,
    pub params: Vec<String>,
    pub handler: fn(Request) -> Response,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Method {
    Delete,
    Post,
    Get,
    Put,
}

impl FromStr for Method {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DELETE" => Ok(Method::Delete),
            "POST" => Ok(Method::Post),
            "GET" => Ok(Method::Get),
            "PUT" => Ok(Method::Put),
            _ => Err(()),
        }
    }
}

impl Route {
    pub fn new(path: String, method: Method, handler: fn(Request) -> Response) -> Route {
        let parts: Vec<&str> = path.split("/").skip(1).collect();
        let mut params: Vec<String> = Vec::new();
        let mut path: Vec<String> = Vec::new();
        
        for char in parts {
            if char.starts_with(":") {
                let param = char.trim_start_matches(":");
                let param  = param.to_string();
                path.push(String::from("*"));
                params.push(param);
            } else {
                path.push(char.to_string());
            }
        }

        Route { path, method, handler, params }
    }
}
