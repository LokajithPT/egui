use std::io::Write;
use std::net::TcpStream;
use std::thread;

use super::types::{DEFAULT_HOST, DEFAULT_PORT};

pub fn send_message(message: String) {
  let addr = format!("{}:{}", DEFAULT_HOST, DEFAULT_PORT);
  thread::spawn(move || {
    if let Ok(mut stream) = TcpStream::connect(&addr) {
      let _ = stream.write_all(message.as_bytes());
    }
  });
}
