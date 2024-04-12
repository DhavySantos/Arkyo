use crate::network::{Request, Response};
use regex::Regex;

#[derive(Clone)]
pub struct Middleware { 
    handler: fn(&mut Request, &mut Response) -> Result<(), ()>, 
    regex: Regex,
    path: String,
}

impl Middleware {
    pub fn new(path: String, handler: fn(&mut Request, &mut Response) -> Result<(), ()>) -> Middleware {
        let regex = Regex::new(r"(:\w+)").unwrap();
        let regex = regex.replace_all(&path, r"([^/]+)");
        let regex = Regex::new(&regex).unwrap();
        Middleware { handler, path, regex }
    }

    pub fn handle(&mut self, request: &mut Request, response: &mut Response) -> Result<(), ()> {
        (self.handler)(request, response)
    }

    pub fn path(&self) -> &String {
        &self.path
    }
}
