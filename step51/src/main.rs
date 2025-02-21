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


    stream.read(&mut buffer).unwrap();

    let file = fs::read_to_string("index.html").unwrap();
    let response = format!("HTTP/1.1 200 OK\r\n\r\nContent-Length: {}\r\n\r\n{}",file.len(),file);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}