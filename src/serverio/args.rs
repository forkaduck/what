use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

pub struct Args {
    pub sockaddr: SocketAddr,
    pub file: String,
}

use log::error;

impl Args {
    pub fn argparse() -> Result<Args, ()> {
        let mut port = 0;
        let mut filepath = String::new();
        let mut ipslice: Vec<u8> = vec![];

        let mut lastarg = String::from("");

        let mut args = std::env::args();
        args.next();

        for i in args {
            match i.as_str() {
                "-h" => {
                    println!(
                        "{}\n{}\n{}\n{}\n{}\n",
                        "server <option> <option parameter> [<option> <option parameter> ...]",
                        "Options:",
                        "       -h          // shows this help section",
                        "       --hv4 <ip>   // the ip address to bind to",
                        "       -p <port>   // the port"
                    );
                    return Err(());
                }

                "--hv4" => (),
                "-p" => (),
                "--file" => (),

                _ => match lastarg.as_str() {
                    "-h" => (),
                    "--hv4" => {
                        let slices: Vec<&str> = i.split('.').collect();

                        for j in (0..slices.len()).rev() {
                            ipslice.push(slices[j].parse::<u8>().unwrap());
                        }
                    }

                    "-p" => {
                        port = i.parse().unwrap();
                    }

                    "--file" => {
                        filepath = i.parse().unwrap();
                    }

                    _ => {
                        error!("Unrecognized option '{}'", lastarg);
                    }
                },
            }
            lastarg = i;
        }

        if port == 0 || ipslice.len() < 3 || filepath.is_empty() {
            error!("Please give at least the socket ip, port to bind to and a file which should be served!");
            error!("-h might help you with this.");
            return Err(());
        }

        let sock = SocketAddr::V4(SocketAddrV4::new(
            Ipv4Addr::new(ipslice[3], ipslice[2], ipslice[1], ipslice[0]),
            port,
        ));

        Ok(Args {
            sockaddr: sock,
            file: filepath,
        })
    }
}
