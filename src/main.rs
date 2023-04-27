use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    // Accept connections from 127.0.0.1:7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // Handle every incoming connections
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    // first unwrap lines, then unwrap next
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    println!("{:#?}", request_line);
    // if get method and http 1.1, return the following
    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("src/hello.html").unwrap();
        let length = contents.len();
        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("src/404.html").unwrap();
        let length = contents.len();
        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    }
}

// fn handle_connection(mut stream: TcpStream) {
//     let buf_reader = BufReader::new(&mut stream);
//     let http_request: Vec<_> = buf_reader
//         .lines()
//         .map(|result| result.unwrap())
//         .take_while(|line| !line.is_empty())
//         .collect();
//     // println!("Request: {:#?}", http_request);
//     let status_line = "HTTP/1.1 200 OK";
//     let contents = fs::read_to_string("src/hello.html").unwrap();
//     let length = contents.len();

//     // \r\n means enter and then newline, this is the newline method for *nix
//     // let response = "HTTP/1.1 200 OK\r\n\r\n";
//     let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
//     // write_all takes [&u8] as params, so response needs to be sent as bytes
//     stream.write_all(response.as_bytes()).unwrap();
// }
