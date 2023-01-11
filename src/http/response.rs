use super::StatusCode;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::net::TcpStream;
use std::io::{Write, Result as IoResult};

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode, 
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response {
            status_code, 
            body
        }
    }

    // using static dispatch by specifying impl Write
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        // The body is in an Option, so we unpack it first
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        write!(
            stream, "HTTP/1.1 {} {}\r\n\r\n{}", 
            self.status_code, 
            self.status_code.reason_phrase(),
            body
        )
    }
}