use crate::network::{Request, Response};
use regex::Regex;

pub type MiddlewareHandler = fn (Request) -> Result<Request, Response>;

#[derive(Clone)]
pub struct Middleware {
    handler: MiddlewareHandler,
    regex: Regex,
    path: String,
}

impl Middleware {

    pub fn new(path: String, handler: MiddlewareHandler) -> Middleware {
        let regex = Regex::new(r"(:\w+)").unwrap();
        let regex = regex.replace_all(&path, r"([^/]+)");
        let regex = Regex::new(&regex).unwrap();

        Middleware { handler, path, regex }
    }

    pub fn handle(&self, request: Request) -> Result<Request, Response> {
        (self.handler)(request)
    }

    pub fn compare(&self, input: &str) -> bool {
        self.regex.is_match(input)
    }

    pub fn path(&self) -> &str { 
        &self.path.as_str()
    } 
}
