use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
// --snip--

const ADDRESS: &str = "127.0.0.1:7878";
const OK_RESPONSE: &str = "HTTP/1.1 200 OK";
const NOT_FOUND_RESPONSE: &str = "HTTP/1.1 404 NOT FOUND";

fn main() {
    let listener = TcpListener::bind(ADDRESS).unwrap_or_else(|error| {
        panic!(
            "failed to bind to {ADDRESS}: {error}. Make sure no other server is using this port"
        );
    });

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let response = build_response(&request_line);

    stream.write_all(response.as_bytes()).unwrap();
}

fn build_response(request_line: &str) -> String {
    let (status_line, file_path) = match request_line {
        "GET / HTTP/1.1" => (OK_RESPONSE, "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(10));
            (OK_RESPONSE, "hello.html")
        }
        _ => (NOT_FOUND_RESPONSE, "404.html"),
    };

    let contents = fs::read_to_string(file_path).unwrap();
    let content_length = contents.len();

    format!("{status_line}\r\nContent-Length: {content_length}\r\n\r\n{contents}")
}
