use std::net::TcpListener;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;
use std::{fmt::format, io::Read};

use super::types::{DEFAULT_HOST, DEFAULT_PORT, MessageTx, ServerState};

#[derive(Debug)]
enum MotorState {
  Busy,
  Sleeping,
  Dead,
  On,
  Off,
  Ready,
}

#[derive(Debug)]
struct motorshit {
  state: MotorState,
  axis: [f32; 3],
  worldloc: [f32; 3],
}

impl motorshit {
  fn new() -> Self {
    Self {
      state: MotorState::Ready,
      axis: [0.1, 0.1, 0.1],
      worldloc: [0.1, 0.1, 0.1],
    }
  }

  fn changestate(&mut self, new_state: MotorState) {
    self.state = new_state;
  }

  fn changeaxis(&mut self, x: f32, y: f32, z: f32) {
    self.axis = [x, y, z];
  }
}

pub fn spawn(tx: MessageTx, state: ServerState) {
  let mut motor = motorshit::new();

  thread::spawn(move || {
    let address = format!("{}:{}", DEFAULT_HOST, DEFAULT_PORT);
    let listener = TcpListener::bind(&address).expect("Failed to bind port");
    listener
      .set_nonblocking(true)
      .expect("Cannot set non-blocking");

    loop {
      if !state.load(Ordering::Relaxed) {
        thread::sleep(Duration::from_millis(100));
        continue;
      }

      match listener.accept() {
        Ok((mut stream, _)) => {
          let mut buffer = [0; 512];
          if let Ok(bytes_read) = stream.read(&mut buffer) {
            let msg = String::from_utf8_lossy(&buffer[..bytes_read])
              .to_lowercase()
              .trim()
              .to_string();

            match msg.as_str() {
              "status" => {
                let frmt = format!(
                  "the status is ... state : {:?} , axis : {:?} , worldloc : {:?} ",
                  motor.state, motor.axis, motor.worldloc
                );
                let _ = tx.send(frmt);
              }
              "vers" => {
                let _ = tx.send("the version is 1.2".to_string());
              }
              "on" | "enable" => {
                motor.changestate(MotorState::On);
                let _ = tx.send("Motor telemetry modified to ON.".to_string());
              }
              "off" | "disable" => {
                motor.changestate(MotorState::Off);
                let _ = tx.send("Motor telemetry modified to OFF.".to_string());
              }

              cmd if cmd.starts_with("axis") => {
                let mut parts = msg.split_whitespace();

                parts.next();

                let x = parts.next().and_then(|s| s.parse::<f32>().ok());

                let y = parts.next().and_then(|s| s.parse::<f32>().ok());

                let z = parts.next().and_then(|s| s.parse::<f32>().ok());

                match (x, y, z) {
                  (Some(val_x), Some(val_y), Some(val_z)) => {
                    motor.changeaxis(val_x, val_y, val_z);
                    let _ = tx.send(format!("Axis is [{}, {}, {}]", val_x, val_y, val_z));
                  }

                  _ => {
                    let _ = tx.send("invalid man ".to_string());
                  }
                }
              }
              _ => {
                let _ = tx.send("i dont see no command like that ".to_string());
              }
            }
          }
        }
        Err(_) => {}
      }
      thread::sleep(Duration::from_millis(10));
    }
  });
}
