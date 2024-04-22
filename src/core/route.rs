use crate::core::Path;
use crate::network::{Method, Request, Response};

pub type Handler = fn(Request) -> Response;

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Route {
    handler: Handler,
    method: Method,
    path: Path,
}

impl Route {
    pub fn new(path: Path, method: Method, handler: fn(Request) -> Response) -> Self {
        Self {
            handler,
            method,
            path,
        }
    }

    #[must_use]
    pub fn compare(&self, input: &str) -> bool {
        self.path.is_exact_match(input)
    }

    #[must_use]
    pub fn handle(&self, request: Request) -> Response {
        (self.handler)(request)
    }

    #[must_use]
    pub fn path(&self) -> &str {
        self.path.as_str()
    }

    #[must_use]
    pub const fn method(&self) -> &Method {
        &self.method
    }
}

#[cfg(test)]
mod tests {}
