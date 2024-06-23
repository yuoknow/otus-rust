use eframe::egui;

use crate::client::Client;
use crate::command::Command;
use crate::response::Response;

mod client;
mod command;
mod response;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "smart-socket-ui",
        eframe::NativeOptions::default(),
        Box::new(|cc| Box::new(SmartSocketApp::new(cc))),
    )
}

struct SmartSocketApp {
    client: Client,
}

impl SmartSocketApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let client = Client::connect("127.0.0.1:55331").expect("Connection error");
        SmartSocketApp { client }
    }
}

impl eframe::App for SmartSocketApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().text_styles.insert(
                egui::TextStyle::Button,
                egui::FontId::new(24.0, eframe::epaint::FontFamily::Proportional),
            );

            let state = match self.client.send(Command::Info).unwrap() {
                Response::Info(is_enabled, power) => (is_enabled, power),
                _ => panic!(),
            };

            ui.heading("Smart Socket Application");
            ui.group(|ui| {
                let status = match state.0 {
                    true => "✔",
                    false => "⊗",
                };
                let status_color = match state.0 {
                    true => egui::Color32::DARK_GREEN,
                    false => egui::Color32::DARK_RED,
                };
                ui.label(
                    egui::RichText::new(format!("Status: {}", status))
                        .size(20.0)
                        .color(status_color),
                );
                ui.label(egui::RichText::new(format!("Power: {:?}", state.1)).size(20.0));
                ui.add_space(8.0);
                let button_label = match state.0 {
                    true => "TURN OFF",
                    false => "TURN ON",
                };

                if ui.button(button_label).clicked() {
                    match state.0 {
                        true => self.client.send(Command::TurnOff),
                        false => self.client.send(Command::TurnOn),
                    }
                    .unwrap();
                }
            });
        });
    }
}
