use crate::network::{Method, Request, Response};
use regex::Regex;

pub type RouteHandler = fn (Request) -> Response;

#[derive(Clone)]
pub struct Route {
    handler: RouteHandler,
    method: Method,
    regex: Regex,
    path: String,
}

impl Route {
    pub fn new(path: String, method: Method, handler: RouteHandler) -> Route {        
        let regex = Regex::new(r"(:\w+)").unwrap();
        let regex = regex.replace_all(&path, r"([^/]+)") + "/?$";
        let regex = Regex::new(&regex).unwrap();

        Route { path, method, handler, regex }
    }

    pub fn compare(&self, input: &str) -> bool { 
        self.regex.is_match(input) 
    }

    pub fn handle(&self, request: Request) -> Response {
        (self.handler)(request)
    }

    pub fn path(&self) -> &str {
        &self.path.as_str()
    }

    pub fn method(&self) -> &Method {
        &self.method
    }
}
