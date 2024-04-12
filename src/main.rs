use arkyo::network::{Method, Request, Response, Status};

fn main() {
    let mut server = arkyo::core::Server::new();
    server.use_route("/usr/:id", Method::Get, usr_handler);
    server.listen("127.0.0.1:8080");
}

fn usr_handler(request: Request) -> Response {
    println!("pass");
    let mut response = Response::new();
    response.body("<h1>hello world</h1>".to_string());
    response
}
