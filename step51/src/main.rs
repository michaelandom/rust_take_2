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
    let mut response = String::new();

    if buffer.starts_with(get){
         response = get_response(String::from("HTTP/1.1 200 OK\r\n"), String::from("index.html"));
   
    } else {
        response = get_response(String::from("HTTP/1.1 404 NOT FOUND\r\n"), String::from("404.html"));
    }

    let (status,file_name) = if buffer.starts_with(get){
        (String::from("HTTP/1.1 200 OK\r\n"),String::from("index.html"))
    } else {
        (String::from("HTTP/1.1 404 NOT FOUND\r\n"),String::from("404.html"))
    };

    response = get_response(status,file_name);


    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}

fn get_response(status: String,file_name: String) -> String{
   let content = fs::read_to_string(file_name).unwrap();
   let response = format!("{}Content-Length: {}\r\n\r\n{}",status,content.len(),content);
    response
}