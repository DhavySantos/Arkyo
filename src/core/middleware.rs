use crate::network::{Request, Response};

pub type MiddlewareHandler = fn (Request) -> Result<Request, Response>;

#[derive(Clone)]
pub struct Middleware {
    handler: MiddlewareHandler,
}

impl Middleware {
    pub fn handle(&self, request: Request) -> Result<Request, Response> {
        (self.handler)(request)
    }
}
