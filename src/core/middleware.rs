use crate::core::Path;
use crate::network::{Request, Response};

pub type Handler = fn(Request) -> Result<Request, Response>;

#[derive(Clone)]
pub struct Middleware {
    handler: Handler,
    path: Path,
    is_static: bool,
}

impl Middleware {
    pub fn new(path: Path, handler: Handler, is_static: bool) -> Self {
        Self {
            handler,
            path,
            is_static,
        }
    }

    /// # Errors
    ///
    /// The current middleware works with a pipeline concept.
    /// This means that if this calls succeeds, another middleware will be called or the main function will. For that, the request must be returned to be used by the following call.
    /// In case the middleware decides the execution should be stopped, it must then return an error with the response to be parsed back to the client.
    pub fn handle(&self, request: Request) -> Result<Request, Response> {
        (self.handler)(request)
    }

    #[must_use]
    pub fn compare(&self, input: &str) -> bool {
        if self.is_static {
            self.path.is_exact_match(input)
        } else {
            self.path.is_match(input)
        }
    }

    #[must_use]
    pub fn path(&self) -> &str {
        self.path.as_str()
    }
}
