use std::net::TcpListener;
use crate::http::Request;
use std::convert::TryFrom;
use std::io::Read;

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

                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            // Parse the bytes as a Request
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {}
                                Err(e) => println!("Failed to parse a request: {}", e)
                            }
                            
                            
                            // let res: &Result<Request, _> = &buffer[..].try_into();
                        }
                        Err(e) => println!("Failed to read from connection : {}", e),
                    }
                },
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }        
    }
}