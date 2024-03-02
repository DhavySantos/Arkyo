use super::response::Response;
use super::request::Request;

#[derive(Debug, Clone)]
pub struct Middleware {
    handler: fn(Request) -> Result<Request,Response>,
}

impl Middleware {

    pub fn new( handler: fn(Request) -> Result<Request, Response>) -> Self {
        Self { handler }
    } 

    pub fn handle(&self, request: Request) -> Result<Request, Response> {
        (self.handler)(request)
    }

}
