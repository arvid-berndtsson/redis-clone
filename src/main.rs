#![allow(unused_imports)]
use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");

                // A buffer to store the command, we do not need to relocate this every time, which is why we declare it outside the loop
                let mut buffer = [0; 512];

                loop {
                    // Read the command from the stream into the buffer
                    let read_count = _stream.read(&mut buffer).unwrap();

                    // If the read count is 0, the client has disconnected, so break out of the loop
                    if read_count == 0 {
                        break;
                    }

                    // Respond to a PING command with PONG
                    _stream.write(b"+PONG\r\n").unwrap();
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
