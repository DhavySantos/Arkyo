use crate::network::{Request, Response};
use crate::core::Path;

pub type MiddlewareHandler = fn (Request) -> Result<Request, Response>;

#[derive(Clone)]
pub struct Middleware {
    handler: MiddlewareHandler,
    path: Path,
    is_static: bool
}

impl Middleware {
    pub fn new(path: Path, handler: MiddlewareHandler, is_static: bool) -> Self {
        Self { handler, path, is_static }
    }

    pub fn handle(&self, request: Request) -> Result<Request, Response> {
        (self.handler)(request)
    }

    #[must_use] pub fn compare(&self, input: &str) -> bool {
        if self.is_static {
            self.path.is_exact_match(input)
        } else {
            self.path.is_match(input)
        }
    }

    #[must_use] pub fn path(&self) -> &str {
        self.path.as_str()
    }
}
