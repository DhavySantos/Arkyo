use crate::network::{Request, Response};
use crate::core::Path;

pub type MiddlewareHandler = fn (Request) -> Result<Request, Response>;

#[derive(Clone)]
pub struct Middleware {
    handler: MiddlewareHandler,
    path: Path,
}

impl Middleware {
    pub fn new(path: Path, handler: MiddlewareHandler) -> Middleware {
        Middleware { handler, path }
    }

    pub fn handle(&self, request: Request) -> Result<Request, Response> {
        (self.handler)(request)
    }

    pub fn compare(&self, input: &str) -> bool {
        self.path.is_match(input)
    }

    pub fn path(&self) -> &str { 
        &self.path.as_str()
    } 
}
