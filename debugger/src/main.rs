mod menu;
mod info;
mod gen_debug;

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
    generator: gen_debug::GeneratorState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            initialized: false,
            state: AppState::Menu,
            prev_state: AppState::Menu,
            generator: gen_debug::GeneratorState::default(),
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
                    self.generator = gen_debug::GeneratorState::default();
                }
                self.prev_state = self.state;
            }
            match self.state {
                AppState::Menu => menu::show(ui, &mut self.state),
                AppState::Generator => gen_debug::show(
                    ui,
                    &mut self.state,
                    &mut self.generator
                ),
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