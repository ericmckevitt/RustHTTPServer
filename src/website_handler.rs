use super::server::Handler;
use super::http::{Request, Response, StatusCode, Method};
use std::fs::{self, canonicalize};

pub struct WebsiteHandler {
    public_path: String
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self {
            public_path
        }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        // Append file_path to public_path
        let path = format!("{}/{}", self.public_path, file_path);

        // Check canonical path to see if it is part of public directory 
        match fs::canonicalize(path) {
            Ok(path) => { 
                if path.starts_with(&self.public_path) {
                    // .ok() converts Result<T, E> to Option<T> and discards any errors
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory Traversal Attack Detected: {}", file_path);
                    None
                }
            }, 
            Err(_) => None,
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                // If method is GET, specify all paths we want to handle
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")), 
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")), 
                path  => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None),
                }
            },
            _  => Response::new(StatusCode::NotFound, None),
        }
    }
}