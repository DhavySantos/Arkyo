use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::Path;
use std::fs::File;

use super::route::Route;
use super::prelude::*;

pub fn handler(mut stream: TcpStream, routes: Vec<Route>, static_folder: Option<String>) {
    let mut buf = [0; 1024];

    let request_str = match stream.read(&mut buf) {
        Ok(size) => String::from_utf8_lossy(&buf[..size]),
        Err(_) => {
            let response = Response::new().status(Status::InternalServerError);
            return stream.write_all(response.to_string().as_bytes()).unwrap()
        },
    };

    let mut request = match Request::new(&request_str) {
        Ok(request) => request,
        Err(_) => {
            let response = Response::new().status(Status::BadRequest);
            return stream.write_all(response.to_string().as_bytes()).unwrap()
        }
    };

    if let Some(static_folder) = &static_folder {
        let path = String::new() + &static_folder + "/" + &request.path.join("/");

        if Path::new(&path).exists() {
            let mut file_string = String::new();
            let mut file = match File::open(&path) {
                Err(_) => {
                    let response = Response::new().status(Status::InternalServerError);
                    return stream.write_all(response.to_string().as_bytes()).unwrap()
                },
                Ok(file) => file,
            };

            file.read_to_string(&mut file_string).unwrap();

            let response = Response::new()
                .status(Status::Ok)
                .body(file_string);

            return stream.write_all(response.to_string().as_bytes()).unwrap();
        }
    }
    
    let mut opt_route: Option<&Route> = None;

    for route in routes.iter() {
        if route.path.len() != request.path.len() || route.method != request.method { continue;}
        
        let mut path: Vec<String> = Vec::new();
        
        for (request_path, route_path) in request.path.iter().zip(route.path.iter()) {
                //println!("{} {}", request_path, route_path);
                if !route_path.starts_with(":") && request_path != route_path { break; }
                
                if route_path.starts_with(":") {
                    let key = route_path[1..].to_string();
                    let value = request_path.to_string();
                    path.push(route_path.to_string());
                    request.params.insert(key, value);
                    continue;
                } 

                if route_path == request_path { 
                    path.push(route_path.to_string());
                    continue; 
                }
        }
        
        if path == route.path { 
            opt_route = Some(route); 
            break; 
        }

    }
    
    let response = match opt_route {
        Some(route) => (route.handler)(request),
        None => {
            let response = Response::new().status(Status::NotFound);
            return stream.write_all(response.to_string().as_bytes()).unwrap()
        },
    };
    stream.write_all(response.to_string().as_bytes()).unwrap();
}