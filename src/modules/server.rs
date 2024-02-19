use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::path::Path;
use std::fs::File;

use super::{response::Response, request::Request, route::{Route, Method}};

pub struct Server { 
    addr: String,
    routes: Vec<Route>,
    static_folder: Option<String>,
}

impl Server {

    pub fn new(addr: &str) -> Server {
        Server { addr: addr.to_string(), routes: Vec::new(), static_folder: None }
    }

    pub fn add_route(&mut self, path: &str, method: Method , handler: fn(Request) -> Response) {
        self.routes.push(Route::new(path.to_string(), method, handler));
    }

    pub fn static_folder(&mut self, path: &str) {
        self.static_folder = Some(path.to_string());
    }

    pub fn listen (&self) {
        let listener = TcpListener::bind(&self.addr).unwrap();

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => self.handle_connection(stream),
                Err(err) => println!("Connection failed: {:?}", err), 
            }   
        }
    }

    fn handle_connection (&self, mut stream: TcpStream) {
        let mut buf = [0; 1024];

        let request_string = match stream.read(&mut buf) {
            Ok(size) => String::from_utf8_lossy(&buf[..size]),
            Err(_) => return stream.write_all(b"HTTP/1.1 500 Internal Server Error\r\n\r\n").unwrap(),
        };

        let request = Request::new(&request_string);
        
        if let Some(static_folder) = &self.static_folder {
            let path = String::new() + &static_folder + &request.path;

            if Path::new(&path).exists() {
                let mut file_string = String::new();
                let mut file = match File::open(&path) {
                    Err(_) => return stream.write_all(b"HTTP/1.1 404 Not Found\r\n\r\n").unwrap(),
                    Ok(file) => file,
                };
    
                file.read_to_string(&mut file_string).unwrap();
    
                let response = Response::new()
                    .status(200)
                    .body(file_string);
    
                return stream.write_all(response.to_string().as_bytes()).unwrap();
            }
        }

        let route = self.routes.iter().find(|route| route.path == request.path && route.method == request.method);
        
        let response = match route {
            Some(route) => (route.handler)(request),
            None => return stream.write_all(b"HTTP/1.1 404 Not Found\r\n\r\n").unwrap(),
        };

        stream.write_all(response.to_string().as_bytes()).unwrap();
    }
}   