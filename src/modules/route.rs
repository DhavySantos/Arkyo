use std::{clone, str::FromStr};

use super::{request::Request, response::Response};

#[derive(Debug, Clone)]
pub struct Route {
    pub path: String,
    pub method: Method,
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
        Route { path, method, handler }
    }
}
