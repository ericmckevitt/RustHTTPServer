use std::net::TcpListener;
use crate::http::{Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::{Read, Write};

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
    pub fn run(self) {
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
                                Ok(request) => {
                                    dbg!(request);
                                    Response::new(
                                        StatusCode::Ok, 
                                        Some("<h1>IT WORKS!!!!</h1>".to_string())
                                    )
                                }
                                Err(e) => {
                                    println!("Failed to parse a request: {}", e);
                                    Response::new(StatusCode::BadRequest, None)
                                }
                            };

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