use crate::http::{Method, Request, Response};
use crate::util::Location;

use std::collections::HashMap;

pub type RouteHandler = fn(&mut Request, &mut Response);

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone)]
pub struct Route {
    handler: RouteHandler,
    location: Location,
    method: Method,
}

impl Route {
    pub fn new(location: Location, method: Method, handler: RouteHandler) -> Self {
        Self {
            location,
            handler,
            method,
        }
    }

    pub fn handle(&self, req: &mut Request, res: &mut Response) {
        (self.handler)(req, res);
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn is_match(&self, location: &str) -> bool {
        self.location.start_with(location)
    }
}
