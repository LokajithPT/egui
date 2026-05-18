use eframe::egui;
use std::sync::atomic::Ordering;
use std::time::Instant;

use crate::models::{LogEntry, MAX_LOGS, MessageDirection};
use crate::network::{client, types::ServerState};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ActiveView {
  TcpLogs,
  Canvas1,
  Canvas2,
  Canvas3,
}

pub struct App {
  input_text: String,
  logs: Vec<LogEntry>,
  receiver: crate::network::MessageRx,
  server_state: ServerState,
  start_time: Instant,
  port: u16,

  //navs
  active_view: ActiveView,
}

impl App {
  pub fn new(rx: crate::network::MessageRx, server_state: ServerState) -> Self {
    Self {
      input_text: String::new(),
      logs: vec![LogEntry::new(
        0,
        "Ready. Click Start Server to begin listening.".to_string(),
        MessageDirection::System,
      )],
      receiver: rx,
      server_state,
      start_time: Instant::now(),
      port: crate::network::DEFAULT_PORT,

      active_view: ActiveView::TcpLogs,
    }
  }

  fn now_ms(&self) -> u64 {
    self.start_time.elapsed().as_millis() as u64
  }

  fn is_running(&self) -> bool {
    self.server_state.load(Ordering::Relaxed)
  }

  fn toggle_server(&mut self) {
    let was_running = self.is_running();

    if was_running {
      self.server_state.store(false, Ordering::Relaxed);
      let now = self.now_ms();
      self.add_log(
        now,
        "Server stopping...".to_string(),
        MessageDirection::System,
      );
    } else {
      self.server_state.store(true, Ordering::Relaxed);
      let now = self.now_ms();
      self.add_log(
        now,
        format!("Server started on 127.0.0.1:{}", self.port),
        MessageDirection::System,
      );
    }
  }

  fn add_log(&mut self, timestamp_ms: u64, message: String, direction: MessageDirection) {
    if self.logs.len() >= MAX_LOGS {
      self.logs.remove(0);
    }
    self
      .logs
      .push(LogEntry::new(timestamp_ms, message, direction));
  }

  fn send_message(&mut self) {
    if self.input_text.is_empty() || !self.is_running() {
      return;
    }

    let msg_to_send = self.input_text.clone();
    let now = self.now_ms();

    client::send_message(msg_to_send.clone());

    self.add_log(now, msg_to_send, MessageDirection::Outgoing);
    self.input_text.clear();
  }
}

impl eframe::App for App {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    let now = self.now_ms();
    let mut new_data = false;

    while let Ok(new_msg) = self.receiver.try_recv() {
      self.add_log(now, new_msg, MessageDirection::Incoming);
      new_data = true;
    }

    self.handle_side_panel(ctx);

    if self.active_view == ActiveView::TcpLogs {
      self.render_toolbar(ctx);

      self.render_input_bar(ctx);
    }

    match self.active_view {
      ActiveView::TcpLogs => {
        self.render_logs(ctx);
      }

      ActiveView::Canvas1 => {
        egui::CentralPanel::default().show(ctx, |ui| {
          ui.heading("canvas 1 ");
          ui.label("this is the first thing ");
        });
      }

      ActiveView::Canvas2 => {
        egui::CentralPanel::default().show(ctx, |ui| {
          ui.heading("canvas 2 ");
          ui.label("this is the second thign ");
        });
      }

      ActiveView::Canvas3 => {
        egui::CentralPanel::default().show(ctx, |ui| {
          ui.heading("canvas 3 ");
          ui.label("this is the third thing ");
        });
      }
    }

    self.handle_repaint(ctx, new_data);
  }
}

impl App {
  fn render_toolbar(&mut self, ctx: &egui::Context) {
    egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
      ui.set_height(48.0);
      ui.horizontal(|ui| {
        ui.add_space(12.0);

        ui.label(egui::RichText::new("TCP Log Lab").size(18.0).strong());

        ui.separator();

        ui.label(
          egui::RichText::new(format!("Port: {}", self.port))
            .size(14.0)
            .color(egui::Color32::GRAY),
        );

        ui.separator();

        let running = self.is_running();
        let (status_text, status_color) = if running {
          ("RUNNING", egui::Color32::from_rgb(46, 204, 113))
        } else {
          ("STOPPED", egui::Color32::from_rgb(149, 165, 166))
        };

        ui.label(
          egui::RichText::new(status_text)
            .size(14.0)
            .color(status_color)
            .strong(),
        );

        ui.separator();

        ui.label(
          egui::RichText::new(format!("{} messages", self.logs.len()))
            .size(14.0)
            .color(egui::Color32::GRAY),
        );

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
          let (btn_text, btn_color) = if running {
            ("Stop Server", egui::Color32::from_rgb(192, 57, 43))
          } else {
            ("Start Server", egui::Color32::from_rgb(39, 174, 96))
          };

          let btn = egui::Button::new(btn_text)
            .fill(btn_color)
            .min_size(egui::vec2(100.0, 28.0));

          if ui.add(btn).clicked() {
            self.toggle_server();
          }

          ui.add_space(8.0);
        });
      });
    });
  }

  fn render_logs(&self, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
      egui::Frame::none().show(ui, |ui| {
        egui::ScrollArea::vertical()
          .stick_to_bottom(true)
          .show(ui, |ui| {
            for log in &self.logs {
              let color = match log.direction {
                MessageDirection::Incoming => egui::Color32::from_rgb(46, 204, 113),
                MessageDirection::Outgoing => egui::Color32::from_rgb(52, 152, 219),
                MessageDirection::System => {
                  if log.message.contains("started") {
                    egui::Color32::from_rgb(39, 174, 96)
                  } else if log.message.contains("stopp") {
                    egui::Color32::from_rgb(192, 57, 43)
                  } else {
                    egui::Color32::from_gray(200)
                  }
                }
              };

              ui.label(
                egui::RichText::new(log.format())
                  .monospace()
                  .size(13.0)
                  .color(color),
              );
            }
          });
      });
    });
  }

  fn render_input_bar(&mut self, ctx: &egui::Context) {
    egui::TopBottomPanel::bottom("input_bar").show(ctx, |ui| {
      ui.set_height(70.0);

      egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.horizontal(|ui| {
          ui.add_space(12.0);

          let running = self.is_running();

          let text_edit = egui::TextEdit::singleline(&mut self.input_text)
            .hint_text("Enter message...")
            .desired_width(480.0);

          let response = ui.add_enabled(running, text_edit);

          if response.lost_focus() && ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
            self.send_message();
          }

          ui.add_space(8.0);

          let send_btn = egui::Button::new("Send")
            .fill(egui::Color32::from_rgb(52, 152, 219))
            .min_size(egui::vec2(80.0, 28.0));

          let send_enabled = running && !self.input_text.is_empty();
          if ui.add_enabled(send_enabled, send_btn).clicked() {
            self.send_message();
          }

          ui.add_space(8.0);

          let clear_btn = egui::Button::new("Clear")
            .fill(egui::Color32::from_rgb(149, 165, 166))
            .min_size(egui::vec2(70.0, 28.0));

          if ui.add(clear_btn).clicked() {
            self.logs.clear();
          }

          ui.add_space(12.0);
        });
      });
    });
  }

  fn handle_repaint(&self, ctx: &egui::Context, new_data: bool) {
    use std::time::Duration;

    if new_data || !self.input_text.is_empty() || ctx.input(|i| !i.events.is_empty()) {
      ctx.request_repaint();
    } else {
      ctx.request_repaint_after(Duration::from_millis(500));
    }
  }
  // pub struct App {
  //     input_text: String,
  //     logs: Vec<LogEntry>,
  //     receiver: crate::network::MessageRx,
  //     server_state: ServerState,
  //     start_time: Instant,
  //     port: u16,
  //
  //
  //     //navs
  //     iscan1open : bool ,
  //     iscan2open : bool ,
  //     iscan3open  : bool ,
  // }
  //
  //

  fn handle_side_panel(&mut self, ctx: &egui::Context) {
    egui::SidePanel::left("navigation_sidebar")
      .resizable(false)
      .default_width(130.0)
      .show(ctx, |ui| {
        ui.add_space(10.0);
        ui.heading("Dashboard");
        ui.separator();
        ui.add_space(10.0);

        ui.vertical_centered_justified(|ui| {
          let tcp_btn = ui.selectable_label(self.active_view == ActiveView::TcpLogs, "TCP Logs");
          if tcp_btn.clicked() {
            self.active_view = ActiveView::TcpLogs;
          }

          // ui.label(egui::RichText::new("Canvases").color(egui::Color32::GRAY));
          // ui.separator();
          ui.add_space(5.0);

          // Canvas 1 Button
          if ui
            .selectable_label(self.active_view == ActiveView::Canvas1, "Canv 1")
            .clicked()
          {
            self.active_view = ActiveView::Canvas1;
          }
          ui.add_space(8.0);

          // Canvas 2 Button
          if ui
            .selectable_label(self.active_view == ActiveView::Canvas2, "Canv 2")
            .clicked()
          {
            self.active_view = ActiveView::Canvas2;
          }
          ui.add_space(8.0);

          // Canvas 3 Button
          if ui
            .selectable_label(self.active_view == ActiveView::Canvas3, "Canv 3")
            .clicked()
          {
            self.active_view = ActiveView::Canvas3;
          }
        });
      });
  }
}
