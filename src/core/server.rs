use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

use crate::core::{RouteHandler, MiddlewareHandler};
use crate::network::{Method, Request, Response};
use crate::core::{Route, Path, Pipeline};


pub struct Server {
    pipeline: Vec<Pipeline>
}

impl Server {
    pub fn new() -> Server {
        Server { pipeline: Vec::new() }
    }

    pub fn use_route(&mut self, path_str: &str, method: Method, handler: RouteHandler) -> Result<(), ()> {
        //match Path::parse(path_str.to_string()) {
        //    Ok(path) => {
        //        let route = Route::new(path, method, handler);
        //        self.pipeline.push(route);
        //        Ok(())
        //    },
        //    Err(_) => Err(()),
        //}
        todo!();
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
            match middleware.handle(request) {
                Ok(_request) => { request = _request; continue; },
                Err(_response) => { break; },
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
