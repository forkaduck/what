use std::{
    fs,
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

use server::ThreadPool;

struct HttpResponse {
    http_ok: &'static str,
}

impl HttpResponse {
    fn new() -> HttpResponse {
        HttpResponse {
            http_ok: "HTTP/1.1 200 OK\r\n",
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let contents = fs::read_to_string("index.html").unwrap();

    let response = format!(
        "{}Content-Length: {}\r\n\r\n{}",
        HttpResponse::new().http_ok,
        contents.len(),
        contents
    );

    stream.write(String::from(response).as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6969").unwrap();
    let pool = ThreadPool::new(10);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

// user accounts with passwords
// sum of money missing
// your account balance
