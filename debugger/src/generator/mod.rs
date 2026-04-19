use eframe::egui;
use crate::AppState;

pub fn show(ui: &mut egui::Ui, state: &mut AppState, seed_string: &mut String) {
    egui::Grid::new("generator_grid").show(ui, |ui| {
        if ui.button("Menu").clicked() {
            *state = AppState::Menu;
        }
        ui.end_row();

        let seed_box = ui.add_sized(
            [125.0, 30.0],
            egui::TextEdit::singleline(seed_string),
        );
        if seed_box.gained_focus() && seed_string == "Seed" {
            seed_string.clear();
        }
        if seed_box.lost_focus() && seed_string.trim().is_empty() {
            *seed_string = "Seed".to_string();
        }
        ui.end_row();
    });
}