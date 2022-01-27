use std::{
    fs,
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

use log::{debug, error, info};

use hyper::http::{Response, StatusCode};

pub mod serverio;
pub mod threadpool;

// Working on
// TODO
// handle SIGINT
// implement clean shutdown

fn handle_connection(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024];

    stream.read(&mut buffer).unwrap();
    debug!("\nRequest:\n {}", String::from_utf8_lossy(&buffer[..]));

    {
        let content = fs::read_to_string("index.html").unwrap();

        let response = Response::builder()
            .status(StatusCode::OK)
            .header("Content-Length", content.len().to_string())
            .header("Content-Type", "text/html")
            .body(content)
            .unwrap();

        let (parts, body) = response.into_parts();

        let mut responseparse: String = format!("HTTP/1.1 {}\r\n", parts.status);

        for i in parts.headers {
            if let Some(first) = i.0 {
                responseparse.push_str(&format!("{:?}: {:?}\r\n", first, i.1)[..]);
            }
        }
        responseparse.push_str(&format!("\r\n\r\n")[..]);

        responseparse.push_str(&body[..]);

        debug!("\nResponse:\n{}", responseparse);

        stream.write(responseparse.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

fn main() {
    env_logger::init();

    if let Ok(args) = serverio::Args::argparse() {
        let _ = match TcpListener::bind(args.sockaddr) {
            Ok(listener) => {
                info!("Bound to {}", args.sockaddr);

                let pool = threadpool::ThreadPool::new(10);

                for stream in listener.incoming() {
                    let stream = stream.unwrap();

                    pool.execute(|| {
                        handle_connection(stream);
                    });
                }
            }
            Err(error) => {
                error!("Failed to bind to {} -> {}", args.sockaddr, error);
            }
        };
    }
}
