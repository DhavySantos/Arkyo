use std::collections::HashMap;

use arkyo::network::{Method, Request, Response};
use arkyo::core::Server;

fn main() {

    let mut server = Server::new();

    server.use_route("/test/:id", Method::Get, home_handler);

    server.listen("127.0.0.1:8080");
}

fn home_handler(request: &mut Request, response: &mut Response) {
    let mut headers: HashMap<String, String> = HashMap::new();
    headers.insert(String::from("Content-Type"), String::from("application/json"));
    
    response.status(arkyo::network::Status::Ok);
    response.body("{\"test\": 200}".to_string());
    response.headers(headers);
    response.send();
}
