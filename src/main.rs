use std::{
    fs,
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

use log::{debug, error, info};

use hyper::http::{Response, StatusCode};
use rand::Rng;

use std::io::Write;
use std::sync::{Arc, Mutex};

pub mod serverio;
pub mod threadpool;

// Working on
// TODO
// handle SIGINT
// implement clean shutdown
// file logging

struct LogFile {
    file: fs::File,
    entry_counter: u32,
}

fn handle_connection(
    mut stream: TcpStream,
    filepath: String,
    rand_ret: bool,
    logfile: Arc<Mutex<LogFile>>,
    max_log_entries: u32,
) {
    let mut buffer: [u8; 1024] = [0; 1024];

    stream.read(&mut buffer).unwrap();
    debug!("\nRequest:\n {}", String::from_utf8_lossy(&buffer[..]));

    // Write to the log file
    let mut logfile = logfile.lock().unwrap();
    if logfile.entry_counter > max_log_entries {
        logfile.file.set_len(0).unwrap();
        logfile.entry_counter = 0;
        debug!("entry_counter reset!");
    }

    logfile.file.write_all(&buffer[..]).unwrap();
    logfile.entry_counter += 1;

    let mut rng = rand::thread_rng();

    let response_code = match rand_ret {
        true => StatusCode::from_u16(rng.gen_range(100, 599)).unwrap(),
        false => StatusCode::OK,
    };

    {
        let content = fs::read_to_string(filepath).unwrap();

        // Build the response from the default file file
        let response = Response::builder()
            .status(response_code)
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

    // Create the log file descriptor
    let logfile = Arc::new(Mutex::new(LogFile {
        file: fs::OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("access.log")
            .unwrap(),
        entry_counter: 0,
    }));

    if let Ok(args) = serverio::Args::argparse() {
        let _ = match TcpListener::bind(args.sockaddr) {
            Ok(listener) => {
                info!("Bound to {}", args.sockaddr);

                let pool = threadpool::ThreadPool::new(10);

                for stream in listener.incoming() {
                    let stream = stream.unwrap();
                    let filepath = args.filepath.clone();
                    let logfile = logfile.clone();
                    let max_log_entries = args.max_log_entries.clone();

                    pool.execute(move || {
                        handle_connection(
                            stream,
                            filepath,
                            args.rand_ret,
                            logfile,
                            max_log_entries,
                        );
                    });
                }
            }
            Err(error) => {
                error!("Failed to bind to {} -> {}", args.sockaddr, error);
            }
        };
    }
}
