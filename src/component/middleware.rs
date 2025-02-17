use crate::http::{Request, Response};
use crate::util::Location;

pub type MiddlewareHandler = fn(&mut Request, &mut Response);

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone)]
pub struct Middleware {
    handler: MiddlewareHandler,
    location: Location,
}

impl Middleware {
    pub fn new(location: Location, handler: MiddlewareHandler) -> Self {
        Self { location, handler }
    }

    pub fn handle(&self, req: &mut Request, res: &mut Response) {
        (self.handler)(req, res)
    }

    pub fn is_match(&self, location: &str) -> bool {
        self.location.start_with(location)
    }
}
