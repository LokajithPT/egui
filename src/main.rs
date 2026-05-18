mod app;
mod models;
mod network;

use eframe::egui;
use std::sync::mpsc;
use std::sync::{Arc, atomic::AtomicBool};

fn main() -> Result<(), eframe::Error> {
  let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default()
      .with_inner_size([750.0, 650.0])
      .with_min_inner_size([600.0, 500.0]),
    ..Default::default()
  };

  let (tx, rx) = mpsc::channel();
  let server_state = Arc::new(AtomicBool::new(false));

  network::server::spawn(tx, server_state.clone());

  eframe::run_native(
    "TCP Log Lab",
    options,
    Box::new(|cc| {
      cc.egui_ctx.set_visuals(egui::Visuals::dark());
      Box::new(app::App::new(rx, server_state))
    }),
  )
}
