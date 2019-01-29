use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};
use std::thread;

use whoami;


fn handle_read(mut stream: &TcpStream) {
    let mut buf = [0u8 ;4096];
    match stream.read(&mut buf) {
        Ok(_) => {
            let req_str = String::from_utf8_lossy(&buf);
            println!("{}", req_str);
            },
        Err(e) => println!("Unable to read stream: {}", e),
    }
}

fn handle_write(mut stream: TcpStream, counter: &u64) {
    let mut http_status = "200 OK";
    if counter > &4u64 {
        http_status = "500 Internal Server Error";
    }
    let hostname = whoami::hostname();
    let response = format!("HTTP/1.1 {}\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello {}</body></html>\r\n", http_status, hostname);
    let response = response.as_bytes();
    match stream.write(response) {
        Ok(_) => println!("Request #{0}, '{1}' response sent\n", counter, http_status),
        Err(e) => println!("Failed sending response: {}", e),
    }
}

fn handle_client(stream: TcpStream, counter: &u64) {
    handle_read(&stream);
    handle_write(stream, &counter);
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    println!("Listening for connections on port {}", 8080);

    let mut counter = 0;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                counter += 1;
                println!("{}", counter);
                thread::spawn(move || {
                     handle_client(stream, &counter)
                });
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}