use miniserver::ThreadPool;
use std::{
    fs,
    io::prelude::*,
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    // Accept connections from 127.0.0.1:7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    // Handle every incoming connections
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap(); //read data from steam to buffer

    let get = b"GET /HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

// fn handle_connection(mut stream: TcpStream) {
//     let buf_reader = BufReader::new(&mut stream);
//     // first unwrap lines, then unwrap next
//     let request_line = buf_reader.lines().next().unwrap().unwrap();

//     println!("{:#?}", request_line);
//     // if get method and http 1.1, return the following
//     // let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
//     //     ("HTTP/1.1 200 OK", "hello.html")
//     // } else {
//     //     ("HTTP/1.1 404 NOT FOUND", "404.html")
//     // };
//     let (status_line, filename) = match &request_line[..] {
//         "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
//         "GET /sleep HTTP/1.1" => {
//             thread::sleep(Duration::from_secs(5));
//             ("HTTP/1.1 200 OK", "hello.html")
//         }
//         _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
//     };

//     let filepath = format!("src/{filename}");
//     let contents = fs::read_to_string(filepath).unwrap();
//     let length = contents.len();
//     let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
//     stream.write_all(response.as_bytes()).unwrap();
// }
