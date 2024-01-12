use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection establisted successfully. {:?}", stream);

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let contents = fs::read_to_string("index.html").unwrap();

        let resp = format! {
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\n\r{}",
            contents.len(),
            contents
        };

        stream.write(resp.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();

        let resp = format! {
            "{}HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\n\r{}",
            status_line,
            contents.len(),
            contents
        };

        stream.write(resp.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}