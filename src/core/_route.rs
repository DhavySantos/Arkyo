use crate::network::{Method, Request, Response};
use regex::Regex;

#[derive(Clone)]
pub struct Route { 
    handler: fn(&mut Request, &mut Response),
    method: Method,
    regex: Regex,
    path: String,
}

impl Route {
    pub fn new(path: String, method: Method, handler: fn(&mut Request, &mut Response)) -> Route {
        let regex = Regex::new(r"(:\w+)").unwrap();
        let regex = regex.replace_all(&path, r"([^/]+)");
        let regex = Regex::new(&regex).unwrap();
        
        Route { path, method, handler, regex }    
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
