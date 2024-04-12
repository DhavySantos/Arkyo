use std::net::{TcpListener, TcpStream};
use std::io::Read;
use regex::Regex;

use crate::core::{Middleware, Pipeline, Route};
use crate::network::{ Request, Response};


pub struct Server { 
    pipes: Vec<Pipeline>
}

impl Server {
    pub fn new() -> Server {
        Server { 
            pipes: Vec::new(),
        }    
    }

    pub fn listen(&self, addr: &str) {
        let listener = match TcpListener::bind(&addr) {
            Err(err) => panic!("{:#?}", err),
            Ok(listener) => listener,
        };

        for incoming in listener.incoming() {
            if let Ok(stream) = incoming {
                let pipes = self.pipes.clone();
                std::thread::spawn(move || handle_connection(stream, pipes));
            };
        };
    }
}

fn handle_connection(mut stream: TcpStream, mut pipes: Vec<Pipeline>) {
    let mut buffer = vec![0; 1024];
    
    let request_str = match stream.read(&mut buffer) {
        Ok(size) => String::from_utf8_lossy(&buffer[..size]),
        Err(err) => panic!("{:#?}", err),
    };

    let mut request = match Request::from_str(&request_str) {
        Err(err) => panic!("{:#?}", err),
        Ok(request) => request,
    };

    let mut response = Response::new(&mut stream);

    for pipe in pipes.iter_mut() {
        if let Pipeline::Middleware(middleware) = pipe {
            match middleware.handle(&mut request, &mut response) {
                Ok(_) => continue,
                Err(_) => break,
            };
        };
    };
}
