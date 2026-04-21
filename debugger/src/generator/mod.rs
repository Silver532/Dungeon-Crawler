use std::collections::VecDeque;

use eframe::egui;
use crate::AppState;

#[derive(PartialEq)]
pub enum Stages {
    Stage1,
    Stage2,
    Stage3,
}

impl std::fmt::Display for Stages {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Stages::Stage1 => write!(f, "Stage 1"),
            Stages::Stage2 => write!(f, "Stage 2"),
            Stages::Stage3 => write!(f, "Stage 3"),
        }
    }
}

pub struct GeneratorState {
    pub seed_string: String,
    pub time_check: bool,
    pub test_count: u16,
    pub selected_stage: Stages,
    pub _recent_seeds: VecDeque<String>,
}

impl Default for GeneratorState {
    fn default() -> Self {
        Self {
            seed_string: "Seed".to_string(),
            time_check: false,
            test_count: 100,
            _recent_seeds: VecDeque::new(),
            selected_stage: Stages::Stage1,
        }
    }
}

pub fn show(
    ui: &mut egui::Ui,
    state: &mut AppState,
    generator: &mut GeneratorState,
) {
    ui.vertical(|ui| {
        if ui.button("Menu").clicked() {
            *state = AppState::Menu;
        }

        ui.add_space(16.0);

        ui.horizontal(|ui| {
            let seed_box = ui.add_sized(
                [125.0, 25.0],
                egui::TextEdit::singleline(&mut generator.seed_string),
            );
            if seed_box.gained_focus() && generator.seed_string == "Seed" {
                generator.seed_string.clear();
            }
            if seed_box.lost_focus() && generator.seed_string.trim().is_empty() {
                generator.seed_string = "Seed".to_string();
            }
            ui.checkbox(&mut generator.time_check, "Time Testing");
        });

        ui.horizontal(|ui| {
            egui::ComboBox::from_id_salt("Stage Select")
                .selected_text(format!("{}", generator.selected_stage))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut generator.selected_stage, Stages::Stage1, "Stage 1");
                    ui.selectable_value(&mut generator.selected_stage, Stages::Stage2, "Stage 2");
                    ui.selectable_value(&mut generator.selected_stage, Stages::Stage3, "Stage 3");
                });
            
            egui::ComboBox::from_id_salt("Test Count")
                .selected_text(format!("Test Count: {}", generator.test_count))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut generator.test_count, 100, "100");
                    ui.selectable_value(&mut generator.test_count, 1000, "1000");
                    ui.selectable_value(&mut generator.test_count, 10000, "10000");
                });
        });
    });
}