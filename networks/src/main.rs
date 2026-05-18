use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client disconnected.");
                break;
            }
            Ok(bytes_read) => {
                match str::from_utf8(&buffer[..bytes_read]) {
                    Ok(text) => {
                        // Trim whitespace and newlines for exact matching
                        let command = text.trim();
                        println!("Received raw data: {}", command);

                        // Use the match expression directly to assign the response
                        let response : &[u8] = match command {
                            "vers" => b"Version is 1.11\n",
                            "on"   => b"System turned ON\n",
                            "off"  => b"System turned OFF\n",
                            _      => b"Unknown command\n", // Catch-all arm
                        };

                        if let Err(e) = stream.write_all(response) {
                            eprintln!("Write error: {}", e);
                            break;
                        }
                        let _ = stream.flush();
                    }
                    Err(_) => {
                        eprintln!("Received non-UTF8 payload.");
                    }
                }
            }
            Err(e) => {
                eprintln!("Read error: {}", e);
                break;
            }
        }
    }
}




fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Raw TCP Server listening on 127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // For a production app, spawn a thread or task here so it doesn't block
                handle_connection(stream);
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
    Ok(())
}
