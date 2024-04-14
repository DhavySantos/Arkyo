use crate::network::{Method, Request, Response};
use super::Path;

#[derive(Clone)]
pub struct Route {
    handler: fn(Request) -> Response,
    method: Method,
    path: Path,
}

impl Route {
    pub fn new(path: Path, method: Method, handler: fn(Request) -> Response) -> Route {
        Route { path, method, handler }
    }

    pub fn compare(&self, input: &str) -> bool { self.path.is_match(input) }

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
