use std::{
    fs,
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

use log::{debug, error, info};

use std::io::Write;
use std::sync::{Arc, Mutex};

pub mod serverio;
pub mod threadpool;

// Working on
// TODO

struct LogFile {
    file: fs::File,
    entry_counter: u32,
}

fn handle_connection(
    mut stream: TcpStream,
    path: String,
    rand_ret: bool,
    logfile: Arc<Mutex<LogFile>>,
    max_log_entries: u32,
) {
    use hyper::http::{Response, StatusCode};
    use rand::Rng;

    let mut buffer: [u8; 8192] = [0; 8192];

    if stream.read(&mut buffer).is_err() {
        debug!("Reading stream failed!");
        return;
    }

    // Write to the log file
    let mut logfile = logfile.lock().unwrap();
    if logfile.entry_counter > max_log_entries {
        logfile.file.set_len(0).unwrap();
        logfile.entry_counter = 0;
        debug!("entry_counter reset!");
    }

    // Prepend the request timestamp
    {
        let mut outbuffer: Vec<u8> = vec![];

        for i in format!("\n\nRequest Timestamp: {}\n", chrono::offset::Utc::now()).as_bytes() {
            outbuffer.push(*i);
        }

        for i in 0..buffer.len() {
            if buffer[i] != 0 {
                outbuffer.push(buffer[i]);
            }
        }

        logfile.file.write_all(&outbuffer).unwrap();
        logfile.entry_counter += 1;
    }

    // Return a random response code if needed
    let mut rng = rand::thread_rng();

    let response_code = match rand_ret {
        true => StatusCode::from_u16(rng.gen_range(100, 599)).unwrap(),
        false => StatusCode::OK,
    };

    {
        let mut return_robots = false;

        // Check if the robots.txt is requested
        {
            let mut count: usize = 0;
            let testcase = "/robots.txt".as_bytes();
            for i in buffer.iter() {
                if *i == testcase[count] && count < testcase.len() {
                    count += 1;
                }

                if count == testcase.len() {
                    return_robots = true;
                    break;
                }
            }
        }

        let content = fs::read_to_string(match return_robots {
            true => path + "/robots.txt",
            false => path + "/index.html",
        })
        .unwrap();

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
    use std::sync::mpsc::channel;

    let (tx, rx) = channel();

    env_logger::init();

    ctrlc::set_handler(move || tx.send(()).unwrap()).unwrap();

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
                listener.set_nonblocking(true).unwrap();

                let pool = threadpool::ThreadPool::new(10);

                loop {
                    match listener.accept() {
                        Ok(stream) => {
                            let path = args.path.clone();
                            let logfile = logfile.clone();
                            let max_log_entries = args.max_log_entries.clone();

                            pool.execute(move || {
                                handle_connection(
                                    stream.0,
                                    path,
                                    args.rand_ret,
                                    logfile,
                                    max_log_entries,
                                );
                            });
                        }
                        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                            use std::{thread, time};

                            if rx.try_recv().ok() == Some(()) {
                                break;
                            }
                            thread::sleep(time::Duration::from_millis(10));
                            continue;
                        }
                        Err(e) => panic!("encountered IO error: {}", e),
                    }
                }
            }
            Err(error) => {
                error!("Failed to bind to {} -> {}", args.sockaddr, error);
            }
        };
    }
}
