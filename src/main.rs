use std::{
    fs,
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

pub mod httpbuilder;
pub mod serverio;
pub mod threadpool;

// TODO
// handle SIGINT
// implement database interface
// implement generic way to create endpoints

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let response = {
        let mut builder = httpbuilder::HttpBuilder::default();
        builder.set_code(200).unwrap();
        builder.set_content(fs::read_to_string("index.html").unwrap());
        builder.build()
    };

    stream.write(String::from(response).as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    if let Ok(args) = serverio::Args::argparse() {
        if let Ok(listener) = TcpListener::bind(args.sockaddr) {
            println!("Bound to {}", args.sockaddr);

            let pool = threadpool::ThreadPool::new(10);

            for stream in listener.incoming() {
                let stream = stream.unwrap();

                pool.execute(|| {
                    handle_connection(stream);
                });
            }
        } else {
            println!("Failed to bind to {}", args.sockaddr);
        }
    }
}

// user accounts with passwords
// sum of money missing
// your account balance
