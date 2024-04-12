use crate::network::{Method, Request, Response};

#[derive(Clone)]
pub struct Route {
    handler: fn(&mut Request, &mut Response),
    method: Method,
    path: String,
}

impl Route {
    pub fn new(path: String, method: Method, handler: fn(&mut Request, &mut Response)) -> Route {
        Route {
            path,
            method,
            handler,
        }
    }

    pub fn handle(&self, request: &mut Request, response: &mut Response) {
        (self.handler)(request, response);
    }

    pub fn path(&self) -> &String {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }
}
