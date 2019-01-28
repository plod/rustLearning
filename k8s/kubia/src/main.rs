use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};
use std::thread;

use chrono::Utc;
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

fn handle_write(mut stream: TcpStream) {
    let request_ts = Utc::now().timestamp();
    let mut status_code = "200";
    if request_ts % 5 == 0 {
        status_code = "500";
    }
    let hostname = whoami::hostname();
    let response = format!("HTTP/1.1 {} OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello {}</body></html>\r\n", status_code, hostname);
    let response = response.as_bytes();
    match stream.write(response) {
        Ok(_) => println!("{} Response sent\n", status_code),
        Err(e) => println!("Failed sending response: {}", e),
    }
}

fn handle_client(stream: TcpStream) {
    handle_read(&stream);
    handle_write(stream);
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    println!("Listening for connections on port {}", 8080);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}