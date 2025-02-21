use std::fs;
use std::net::TcpListener;

use std::net::TcpStream;

use std::io::prelude::*;
fn main() {


    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("here");
    for stream in listener.incoming()  {
        let stream = stream.unwrap();
        handle_connection(stream);
    }

    println!("last");



}


fn handle_connection(mut stream : TcpStream) {

    let mut  buffer = [0;1024];


    let get = b"GET / HTTP/1.1\r\n";
    stream.read(&mut buffer).unwrap();
    let mut content = String::new();
    let mut response = String::new();

    if buffer.starts_with(get){
         content = fs::read_to_string("index.html").unwrap();
         response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",content.len(),content);
    } else {
        content = fs::read_to_string("404.html").unwrap();
        response = format!("HTTP/1.1 404 OK\r\nContent-Length: {}\r\n\r\n{}",content.len(),content);
    }


    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}