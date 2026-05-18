pub const MAX_LOGS: usize = 500;

pub struct LogEntry {
  pub timestamp_ms: u64,
  pub message: String,
  pub direction: MessageDirection,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MessageDirection {
  Incoming,
  Outgoing,
  System,
}

impl LogEntry {
  pub fn new(timestamp_ms: u64, message: String, direction: MessageDirection) -> Self {
    Self {
      timestamp_ms,
      message,
      direction,
    }
  }

  pub fn format(&self) -> String {
    let prefix = match self.direction {
      MessageDirection::Incoming => "[IN] ",
      MessageDirection::Outgoing => "[OUT] ",
      MessageDirection::System => "",
    };

    let minutes = self.timestamp_ms / 60000;
    let seconds = (self.timestamp_ms / 1000) % 60;
    let millis = self.timestamp_ms % 1000;

    format!(
      "[{:02}:{:02}.{:03}] {}{}",
      minutes, seconds, millis, prefix, self.message
    )
  }
}
