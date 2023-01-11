use std::net::TcpListener;
use crate::http::{Request, Response, StatusCode, ParseError};
use std::convert::TryFrom;
use std::io::Read;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    // Associated function new()
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    // Method run(). Methods accept self
    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        // Infinite loop
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {

                    let mut buffer = [0; 1024];

                    // Match on result from parsing request
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            // Parse the bytes as a Request
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e)
                            };

                            // If the response fails to send, print the error
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                            
                            
                            // let res: &Result<Request, _> = &buffer[..].try_into();
                        }
                        // Case that we fail to
                        Err(e) => println!("Failed to read from connection : {}", e),
                    }
                },
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }        
    }
}