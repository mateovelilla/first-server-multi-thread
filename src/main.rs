use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); //Return an instance of TCP and is bind beacause "binding to a port"
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
        println!("Connection established!");
    }
}
fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    // we convert the bytes in the buffer to a string and print that string.
    // The String::from_utf8_lossy function takes a &[u8] and produces a String from it.
    // The “lossy” part of the name indicates the behavior of this function when it sees
    // an invalid UTF-8 sequence: it will replace the invalid sequence with �,
    // the U+FFFD REPLACEMENT CHARACTER.
    // You might see replacement characters for characters in the buffer that aren’t
    // filled by request data.
    // println!("Reques: {}",String::from_utf8_lossy(&buffer[..]));
    let get = b"GET / HTTP/1.1\r\n";
    
    // conditional to define if the request is GET or the other type and asign to destructuring
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
   
    // Read HTML file
    let contents = fs::read_to_string(filename).unwrap();
   
    // Build response
    // Option 1:
    // let response = "HTTP/1.1 200 OK\r\n\r\n";
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap()
}
