#![allow(unused_imports)]
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    
    println!("Logs from your program will appear here!");

    // Bind the TCP listener to the specified address and port
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    
    loop { // Infinite loop to accept incoming connections
        let stream = listener.accept().await; // Wait for a new connection
    
        match stream { // Match on the result of accepting a connection
            Ok((mut _stream, _)) => { // If successful, destructure the stream and the address
                println!("accepted new connection");

                // Spawn a new asynchronous task to handle the connection
                tokio::spawn(async move {
                    let mut buf = [0; 512]; // Buffer to hold incoming data
                    loop { // Loop to read data from the stream
                        let read_count = _stream.read(&mut buf).await.unwrap(); // Read data into the buffer
                        if read_count == 0 { // If no data is read, the connection is closed
                            break;
                        }
        
                        // Write a response back to the client
                        _stream.write(b"+PONG\r\n").await.unwrap();
                    }
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
