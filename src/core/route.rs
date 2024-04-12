use crate::network::{Method, Request, Response};
use regex::Regex;

#[derive(Clone)]
pub struct Route { 
    handler: fn(Request) -> Response,
    method: Method,
    regex: Regex,
    path: String,
}

impl Route {
    fn new(path: String, method: Method, handler: fn(Request) -> Response) -> Route {
        let regex = Regex::new(r"(:\w+)").unwrap();
        let regex = regex.replace_all(&path, r"([^/]+)") + "/?$";
        let regex = Regex::new(&regex).unwrap();
        
        Route { path, method, handler, regex }    
    }
    
    pub fn parse(path: String, method: Method, handler: fn(Request) -> Response) -> Result<Route, ()> {
        Ok(Route::new(path, method, handler))
    }

    pub fn compare(&self, input: &String) -> bool { self.regex.is_match(input) }

    pub fn handle(&self, request: Request) -> Response {
        (self.handler)(request)
    }

    pub fn path(&self) -> &String {
        &self.path
    }

    pub fn method(&self) -> &Method { 
        &self.method
    }
}
