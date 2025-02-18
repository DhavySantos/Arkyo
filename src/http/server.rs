use crate::component::{Middleware, Pipeline, Route};
use crate::http::{Method, Request, Response};
use crate::util::Location;

use std::error::Error;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Default)]
pub struct Server {
    pipeline: Vec<Pipeline>,
}

type Handler = fn(&mut Request, &mut Response);

impl Server {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get<T: Fn(&mut Request, &mut Response) + 'static>(
        &mut self,
        location: impl Into<String>,
        callback: T,
    ) -> Result<(), Box<dyn Error>> {
        self.add_route(location, Method::GET, callback)
    }

    pub fn post<T: Fn(&mut Request, &mut Response) + 'static>(
        &mut self,
        location: impl Into<String>,
        callback: T,
    ) -> Result<(), Box<dyn Error>> {
        self.add_route(location, Method::POST, callback)
    }

    pub fn patch<T: Fn(&mut Request, &mut Response) + 'static>(
        &mut self,
        location: impl Into<String>,
        callback: T,
    ) -> Result<(), Box<dyn Error>> {
        self.add_route(location, Method::PATCH, callback)
    }

    pub fn delete<T: Fn(&mut Request, &mut Response) + 'static>(
        &mut self,
        location: impl Into<String>,
        callback: T,
    ) -> Result<(), Box<dyn Error>> {
        self.add_route(location, Method::DELETE, callback)
    }

    pub fn add_route<T: Fn(&mut Request, &mut Response) + 'static>(
        &mut self,
        location: impl Into<String>,
        method: Method,
        callback: T,
    ) -> Result<(), Box<dyn Error>> {
        let location = Location::new(location.into())?;
        let route = Route::new(location, method, callback);
        self.pipeline.push(Pipeline::Route(route));
        Ok(())
    }

    pub fn add_middleware<T: Fn(&mut Request, &mut Response) + 'static>(
        &mut self,
        location: impl Into<String>,
        callback: T,
    ) -> Result<(), Box<dyn Error>> {
        let location = Location::new(location.into())?;
        let middleware = Middleware::new(location, callback);
        self.pipeline.push(Pipeline::Middleware(middleware));
        Ok(())
    }

    pub fn listen(&self, addr: impl AsRef<str>) {
        let listener = match TcpListener::bind(addr.as_ref()) {
            Err(err) => panic!("Failed To Bind at: {} \n{err}", addr.as_ref()),
            Ok(listener) => listener,
        };

        for incoming in listener.incoming() {
            let pipeline = self.pipeline.clone();

            if let Ok(stream) = incoming {
                handle_incoming(stream, pipeline).ok();
            }
        }
    }
}

fn handle_incoming(mut stream: TcpStream, pipeline: Vec<Pipeline>) -> Result<(), Box<dyn Error>> {
    let mut buffer = vec![0; 1024];

    let size = stream.read(&mut buffer)?;
    let request_string = String::from_utf8_lossy(&buffer[..size]);
    let mut request = Request::from(request_string)?;
    let mut response = Response::new(stream);

    for component in pipeline {
        match component {
            Pipeline::Middleware(middleware) => {
                if middleware.is_match(&request.location) {
                    middleware.handle(&mut request, &mut response);
                };
            }

            Pipeline::Route(route) => {
                if route.is_match(&request.location) {
                    if route.method() == request.method() {
                        route.handle(&mut request, &mut response);
                        break;
                    };
                };
            }
        };
    }

    Ok(())
}
