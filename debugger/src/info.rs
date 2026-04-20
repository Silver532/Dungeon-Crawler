use eframe::egui;
use crate::AppState;

pub fn show(ui: &mut egui::Ui, state: &mut AppState) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Dungeon Crawler Control Panel v0.1").size(24.0).strong());
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("Menu").clicked() {
                *state = AppState::Menu
            }
        });
    });

    egui::Grid::new("generator_grid").show(ui, |ui| {
        ui.label("Creator: Silver532");
        ui.end_row();
    });
}