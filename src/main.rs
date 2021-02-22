use std::{
    fs,
    io::prelude::*,
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
};

pub mod builders;
pub mod serverio;
pub mod threadpool;

// Working on
// TODO
// handle SIGINT
// implement database interface
// implement clean shutdown

const ENDPOINTS: &'static [&'static builders::EndpointFace]  = &[&builders::EndpointFace {
        uri: "/test",
        requirements: &["test"],
        handler: test,
    }];


fn test(data: serde_json::Value) -> Result<(), ()> {
    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024];

    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    /*for i in endpoints {
        let payload = 

        let data = serde_json::from_str()
        i.check();
t   }*/

    let response = {
        let mut builder = builders::HttpBuilder::default();
        builder.set_code(200).unwrap();
        builder.set_content(fs::read_to_string("index.html").unwrap(), "text/html".to_string());
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
