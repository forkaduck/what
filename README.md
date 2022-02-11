# What?
is a small webserver honeypot which just logs webrequests and returns 200 on every request with a default page.

## Usage
```
server <option> <option parameter> [<option> <option parameter> ...]
Options:
       -h            // shows this help section
       --hv4 <ip>    // the ip address to bind to
       -p <port>     // the port
       --path <path> // the path which contains an index.html and a robots.txt
       --random-ret  // changes the http return code from 200 to a random number (100 - 599)
       --max-log-entries   // max amount of entries to log in the access.log file before it is reset
```

## Building
```
$ cd what
$ cargo build
$ ./target/debug/server --hv4 127.0.0.1 -p 8080 --file index.html
```
