use eframe::egui;

use crate::AppState;

pub fn show(ui: &mut egui::Ui, state: &mut AppState) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Dungeon Crawler Control Panel v0.1").size(24.0).strong());
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("Info").clicked() {
                // DOES NOTHING RIGHT NOW -- Add Info Panel
            }
        });
    });
    egui::Grid::new("menu_grid").show(ui, |ui| {
        ui.label(egui::RichText::new("Debug Tools").size(16.0).strong());
        ui.end_row();

        if ui.button("Generator").clicked() {
            *state = AppState::Generator
        }
        ui.end_row();
    });
}