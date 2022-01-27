# What?
is a small webserver honeypot which just logs webrequests and returns 200 on every request with a default page.

## Usage
```
server <option> <option parameter> [<option> <option parameter> ...]
Options:
       -h            // shows this help section
       --hv4 <ip>    // the ip address to bind to
       -p <port>     // the port
       --file <file> // the default landing page
```

## Building
```
$ cd what
$ cargo build
$ ./target/debug/server --hv4 127.0.0.1 -p 8080 --file index.html
```
