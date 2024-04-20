use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use crate::core::{Path, Route, Pipeline, Middleware};
use crate::core::{RouteHandler, MiddlewareHandler};
use crate::network::{Method, Request, Response};
use crate::core::path::Errors as PathErrors;

pub enum Errors {
    Path(PathErrors),
}

pub struct Server {
    routes: Vec<Route>,
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}

impl Server {
    #[must_use]
    pub const fn new() -> Self {
        Self { routes: Vec::new() }
    }

    pub fn use_route(
        &mut self,
        path_str: &str,
        method: Method,
        handler: fn(Request) -> Response,
    ) -> Result<(), Errors> {
        match Path::parse(path_str.to_string()) {
            Ok(path) => {
                let route = Route::new(path, method, handler);
                self.routes.push(route);
                Ok(())
            }
            Err(e) => Err(Errors::Path(e)),
        }
    pipeline: Vec<Pipeline>
}

impl Server {
    pub fn new() -> Server {
        Server { pipeline: Vec::new() }
    }


    pub fn listen(&self, addr: &str) {
        let listener = match TcpListener::bind(addr) {
            Err(err) => todo!("{err:#?}"),
            Ok(listener) => listener,
        };

        for incoming in listener.incoming() {
            if let Ok(stream) = incoming {
                let pipeline = self.pipeline.clone();
                std::thread::spawn(move || handle_connection(stream, pipeline));
            };
        }
    }
}

fn handle_connection(mut stream: TcpStream, pipeline: Vec<Pipeline>) {
    let mut buffer = vec![0; 1024];

    let request_str = match stream.read(&mut buffer) {
        Ok(size) => String::from_utf8_lossy(&buffer[..size]),
        Err(err) => todo!("{err:#?}"),
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
