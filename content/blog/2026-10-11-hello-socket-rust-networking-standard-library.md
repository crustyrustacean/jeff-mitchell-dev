+++
title = "Hello, Socket: Rust Networking with the Standard Library"
date = 2026-10-11
description = "Build your first TCP server in Rust using only the standard library — form a socket address, bind a listener, and accept incoming connections."
categories = ["Fundamentals"]
tags = ["rust", "std", "tcp", "networking"]
draft = false
aliases = ["/2026-10-11-hello-socket-rust-networking-standard-library"]
+++

Building network-aware applications in Rust, even at a low level, is pretty straightforward. The standard library gives you the building blocks you need.

### Introduction

Rust is a low-level language, but you can choose your level of low. The standard library affords several tools that make your networking journey easy to start. In this quick tour, we're going to:

- learn what elements from the standard library we can use
- learn how to form a socket address and bind a listener to it
- learn how to accept incoming connections on that listener
- learn how to handle errors, at least in a rudimentary way
- understand where to go to build out from this foundation

Let's go!

### The Code

Here is the code, then I'll break it down step by step:

```Rust
// src/main.rs

// dependencies
use std::net::{Ipv4Addr, TcpListener};

// main function
fn main() {
    // create an IPv4 address
    let address = Ipv4Addr::new(127, 0, 0, 1);

    // create port
    let port: u16 = 8000;

    // create a socket address by pairing the address and port we just made
    let socket_address = (address, port);

    // create a listener and bind it to the socket address we just made, panic and crash out if there's an error
    // returned by the bind method
    let listener = match TcpListener::bind(socket_address) {
        Ok(ltnr) => ltnr,
        Err(e) => panic!(
            "Unable to obtain a listener on which to receive incoming connections: {}",
            e
        ),
    };

    // start a loop and accept incoming connections, match on errors and print them to stderr
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("Connection established!");
                // we don't use the stream yet; that's the subject of a future post
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}
```

Alright, what are we doing here?

- bring into scope some dependencies from the standard library, namely:
  - the struct type Ipv4Addr, gives us tools to create an IPv4 address
  - the TcpListener type
- we then start up a main function and create an Ipv4 address which we bind to a variable called `address`
- we create a `u16` type to represent a port, with the value 8000, and bind to a variable called `port`
- a socket address is formed by pairing the address and port together in a tuple. The `bind` method accepts anything that can be converted into socket addresses, and an `(address, port)` pair is one of them
- armed with a socket address, we create a `listener` by using the `bind()` method available for the `TcpListener` type. The `bind` method takes our socket address as an argument, and the listener it returns is our actual socket — the thing the operating system uses to receive connections
- the bind method can fail, so we match on it
  - success (the `Ok` arm of the match statement) binds the returned listener to a variable called `ltnr`
  - the error path (the `Err` arm of the match statement) causes the program to panic with an error message. In reality, you'll want to handle errors potentially more gracefully. Although there's something to be said for just quitting, because without a listener to receive data, there's not much point in continuing
- now that we have something to listen on, we start up a loop and use the `incoming()` method on our listener, which accepts connections in a blocking manner
- we bind each incoming connection to a variable called `stream`
- we have to match here again, because each connection attempt can fail, even though the `incoming()` iterator itself doesn't error
  - success (the `Ok` arm) we just print out a message confirming we've established a connection. Notice we bind the stream as `_stream` — the leading underscore tells Rust we're intentionally not using the value yet, which keeps the compiler from warning us about an unused variable. When we put the stream to work in a future post, we'll drop the underscore and bind it as `stream`
  - the error path (the `Err` arm) we print out a message to `stderr` giving a reason why the connection failed

### Where to Go Next

This program we've created isn't particularly useful. Yes, we accept incoming connections and we somewhat handle errors, but we don't do anything with the information coming to us. Where can we go from here?

Well, we can build out some functions which take the stream of data and do something with it. In a future article, I'll illustrate how to do that.

I alluded above to the fact this code is blocking. This means every request will block the next until the first is fully processed. You can extend this code by creating a simple event loop, using the `tokio` crate, to handle connections in an asynchronous manner.

### Conclusion

Thanks for reading, I hope you see now that networking in Rust is easy to get started. The standard library affords just enough so that we're not needing to implement a TCP stack from scratch.

If you have any feedback for me on this piece, feel free to sound off in the comments.

### Resources

- [Final Project: Building a Multithreaded Web Server](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)
- [Rust Standard Library: Ipv4Addr](https://doc.rust-lang.org/std/net/struct.Ipv4Addr.html)
- [Rust Standard Library: TcpListener](https://doc.rust-lang.org/std/net/struct.TcpListener.html)
