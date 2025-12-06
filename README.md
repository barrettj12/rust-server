# rust-server
This is a very basic implementation of an HTTP server that I wrote in Rust
while I was learning the language.

To run the server, navigate to the root directory and execute:
```
cargo run
```

The server will randomly be assigned a port, and it will print to the console:
```
listening on 127.0.0.1:<port>
```

Make a request to this address (either using curl or by copy-pasting into your
browser), and it should return a 200 OK response with the body
```
Hello world!
```