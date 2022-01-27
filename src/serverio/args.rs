use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

pub struct Args {
    pub sockaddr: SocketAddr,
    pub filepath: String,
    pub rand_ret: bool,
}

use log::error;

impl Args {
    pub fn argparse() -> Result<Args, ()> {
        let mut arg = Args {
            sockaddr: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080)),
            filepath: String::new(),
            rand_ret: false,
        };

        let mut port = 0;
        let mut ipslice: Vec<u8> = vec![];

        let mut lastarg = String::from("");
        let mut args = std::env::args();
        args.next();

        for i in args {
            match i.as_str() {
                "-h" => {
                    println!(concat!(
                        "server <option> <option parameter> [<option> <option parameter> ...]",
                        "Options:",
                        "       -h            // shows this help section",
                        "       --hv4 <ip>    // the ip address to bind to",
                        "       -p <port>     // the port",
                        "       --file <file> // the default landing page",
                        "       --random-ret  // changes the http return code from 200 to a random number (100 - 599)",
                    ));
                    return Err(());
                }

                "--hv4" => (),
                "-p" => (),
                "--file" => (),
                "--random-ret" => (),

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
                        arg.filepath = i.parse().unwrap();
                    }

                    "--random-ret" => {
                        arg.rand_ret = true;
                    }

                    _ => {
                        error!("Unrecognized option '{}'", lastarg);
                    }
                },
            }
            lastarg = i;
        }

        if port == 0 || ipslice.len() < 3 || arg.filepath.is_empty() {
            error!("Please give at least the socket ip, port to bind to and a file which should be served!");
            error!("-h might help you with this.");
            return Err(());
        }

        arg.sockaddr = SocketAddr::V4(SocketAddrV4::new(
            Ipv4Addr::new(ipslice[3], ipslice[2], ipslice[1], ipslice[0]),
            port,
        ));

        Ok(arg)
    }
}
