use crate::component::{Middleware, MiddlewareHandler, Pipeline, Route, RouteHandler};
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

impl Server {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_route(
        &mut self,
        location: impl Into<String>,
        method: Method,
        handler: RouteHandler,
    ) -> Result<(), Box<dyn Error>> {
        let location = Location::new(location.into())?;
        let route = Route::new(location, method, handler);
        self.pipeline.push(Pipeline::Route(route));
        Ok(())
    }

    pub fn add_middleware(
        &mut self,
        location: impl Into<String>,
        handler: MiddlewareHandler,
    ) -> Result<(), Box<dyn Error>> {
        let location = Location::new(location.into())?;
        let middleware = Middleware::new(location, handler);
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

            match incoming {
                Ok(stream) => handle_incoming(stream, pipeline),
                Err(err) => continue,
            };
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
