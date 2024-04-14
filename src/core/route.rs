use crate::network::{Method, Request, Response};
use regex::Regex;
use super::Path;

#[derive(Clone)]
pub struct Route {
    handler: fn(Request) -> Response,
    method: Method,
    regex: Regex,
    path: Path,
}

impl Route {
    pub fn new(path: Path, method: Method, handler: fn(Request) -> Response) -> Route {
        let regex = Regex::new(r"(:\w+)").unwrap();
        let regex = regex.replace_all(path.as_str(), r"([^/]+)") + "/?$";
        let regex = Regex::new(&regex).unwrap();

        Route { path, method, handler, regex }
    }

    pub fn compare(&self, input: &String) -> bool { self.regex.is_match(input) }

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
