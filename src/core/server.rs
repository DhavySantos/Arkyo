use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

use crate::core::{RouteHandler, MiddlewareHandler};
use crate::network::{Method, Request};
use crate::core::{Route, Pipeline};

use super::Middleware;


pub struct Server {
    pipeline: Vec<Pipeline>
}

impl Server {
    pub fn new() -> Server {
        Server { pipeline: Vec::new() }
    }

    pub fn use_route(&mut self, path_str: &str, method: Method, handler: RouteHandler) -> Result<(), ()> {
        let route = Route::new(String::from(path_str), method, handler);
        self.pipeline.push(Pipeline::Route(route));
        Ok(())
    }

    pub fn use_middleware(&mut self, path_str: &str, handler: MiddlewareHandler) -> Result<(), ()> {
        let middleware = Middleware::new(String::from(path_str), handler);
        self.pipeline.push(Pipeline::Middleware(middleware));
        Ok(())
    }

    pub fn listen(&self, addr: &str) {
        let listener = match TcpListener::bind(&addr) {
            Err(err) => panic!("{:#?}", err),
            Ok(listener) => listener,
        };

        for incoming in listener.incoming() {
            if let Ok(stream) = incoming {
                let pipeline = self.pipeline.clone();
                std::thread::spawn(move || handle_connection(stream, pipeline));
            };
        };
    }
}

fn handle_connection(mut stream: TcpStream, pipeline: Vec<Pipeline>) {
    let mut buffer = vec![0; 1024];

    let request_str = match stream.read(&mut buffer) {
        Ok(size) => String::from_utf8_lossy(&buffer[..size]),
        Err(err) => panic!("{:#?}", err),
    };

    let mut request = match Request::from_str(&request_str) {
        Err(err) => panic!("{:#?}", err),
        Ok(request) => request,
    };

    for pipe in pipeline {
        if let Pipeline::Middleware(middleware) = pipe {
            if !middleware.compare(&request.path()) { continue; }
            match middleware.handle(request) {
                Ok(_request) => { request = _request; continue; },
                Err(_response) => { stream.write_all(_response.to_string().as_bytes()).unwrap(); break; },
            }
        };

        if let Pipeline::Route(route) = pipe {
            if route.method() != request.method() { continue; }
            if !route.compare(request.path()) { continue; };
            let response = route.handle(request);
            stream.write_all(response.to_string().as_bytes()).unwrap();
            break;
        };
    };
}
