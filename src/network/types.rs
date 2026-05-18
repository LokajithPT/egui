use std::sync::Arc;
use std::sync::atomic::AtomicBool;

pub type ServerState = Arc<AtomicBool>;
pub type MessageTx = std::sync::mpsc::Sender<String>;
pub type MessageRx = std::sync::mpsc::Receiver<String>;

pub const DEFAULT_PORT: u16 = 8080;
pub const DEFAULT_HOST: &str = "127.0.0.1";
