use std::str::FromStr;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

use crate::network::{Method, Request, Response};
use crate::core::{Route, Path};


pub struct Server {
    routes: Vec<Route>
}

impl Server {
    pub fn new() -> Server {
        Server { routes: Vec::new() }
    }

    pub fn use_route(&mut self, path_str: &str, method: Method, handler: fn(Request) -> Response) -> Result<(), ()> {
        match Path::parse(path_str.to_string()) {
            Ok(path) => {
                let route = Route::new(path, method, handler);
                self.routes.push(route);
                Ok(())
            },
            Err(_) => Err(()),
        }
    }

    pub fn listen(&self, addr: &str) {
        let listener = match TcpListener::bind(&addr) {
            Err(err) => todo!(),
            Ok(listener) => listener,
        };

        for incoming in listener.incoming() {
            if let Ok(stream) = incoming {
                let routes = self.routes.clone();
                std::thread::spawn(move || handle_connection(stream, routes));
            };
        };
    }
}

fn handle_connection(mut stream: TcpStream, mut routes: Vec<Route>) {
    let mut buffer = vec![0; 1024];

    let request_str = match stream.read(&mut buffer) {
        Ok(size) => String::from_utf8_lossy(&buffer[..size]),
        Err(err) => todo!(),
    };

    let request = match Request::from_str(&request_str) {
        Err(err) => todo!(),
        Ok(request) => request,
    };

    for route in routes.iter_mut() {
        if route.method() != request.method() { continue; }
        if !route.compare(request.path()) { continue; };
        let response = route.handle(request);
        stream.write_all(response.to_string().as_bytes()).unwrap();
        break;
    };
}
