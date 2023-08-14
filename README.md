# Rust Implementation of an HTTP Server
By Eric McKevitt

---

 - Implemented a TCP Listener.
 - Cleaned data on a buffer to discard invalid UTF-8.
 - Worked with lifetime specifiers.
 - Converted from sending via TcpStream to any supporting struct that implements the “Write” trait to promote server flexibility in production and in unit testing. 
 - Implemented custom trait for a Handler in order to handle path validation and serve correct files and HTTP responses for a given request. 
 - Added canonical path validation to prevent directory traversal vulnerabilities.
