mod menu;
mod info;
mod generator;

use eframe::{egui};

#[derive(PartialEq, Clone, Copy)]
enum AppState {
    Menu,
    Generator,
    Info,
}

struct App {
    initialized: bool,
    state: AppState,
    prev_state: AppState,
    seed_string: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            initialized: false,
            state: AppState::Menu,
            prev_state: AppState::Menu,
            seed_string: "Seed".to_string(),
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if !self.initialized {
            ui.ctx().set_visuals(egui::Visuals {
                panel_fill: egui::Color32::from_rgb(30, 30, 30),
                ..egui::Visuals::dark()
            });
            self.initialized = true;
        }
        egui::CentralPanel::default().show_inside(ui, |ui| {
            if self.state != self.prev_state {
                if self.state == AppState::Generator {
                    self.seed_string = "Seed".to_string();
                }
                self.prev_state = self.state;
            }
            match self.state {
                AppState::Menu => menu::show(ui, &mut self.state),
                AppState::Generator => generator::show(ui, &mut self.state, &mut self.seed_string),
                AppState::Info => info::show(ui, &mut self.state),
            }
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Debugger",
        options,
        Box::new(|_cc| Ok(Box::<App>::default())))
}