mod visualizer;

use std::collections::VecDeque;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use eframe::egui::mutex::Mutex;
use fnv::FnvHasher;

use eframe::egui;
use generator::{run_stage_1, run_stage_2};
use crate::AppState;

#[derive(PartialEq, Clone, Copy)]
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
    pub recent_seeds: VecDeque<String>,
    pub active_viewports: Arc<Mutex<Vec<(u64, Stages)>>>,
}

impl Default for GeneratorState {
    fn default() -> Self {
        Self {
            seed_string: "Seed".to_string(),
            time_check: false,
            test_count: 100,
            recent_seeds: VecDeque::new(),
            selected_stage: Stages::Stage1,
            active_viewports: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

fn generate_seed(input: Option<&str>) -> u64 {
    match input {
        Some(s) => {
            let mut hasher: FnvHasher = FnvHasher::default();
            s.hash(&mut hasher);
            hasher.finish()
        }
        None => rand::random::<u64>()
    }
}

fn time_test(stage: &Stages, count: u16) {
    match stage {
        Stages::Stage1 => {
            for _ in 0..count {
                let seed: u64 = rand::random::<u64>();
                _ = run_stage_1(seed)
                //Do something with time testing here
            }
        },
        Stages::Stage2 => {
            for _ in 0..count {
                let seed: u64 = rand::random::<u64>();
                _ = run_stage_2(seed)
                //Do something with time testing here
            }
        },
        Stages::Stage3 => {
            for _ in 0..count {
                //let seed = rand::random::<u64>();
                //_ = run_stage_3(seed)
                //Do something with time testing here
            }
        },
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

        ui.horizontal(|ui| {
            if ui.button("Run").clicked() {
                let len = generator.recent_seeds.len();
                if len >= 10 {
                    generator.recent_seeds.pop_back();
                }
                let seed_string = generator.seed_string.trim();
                let seed_num = match seed_string {
                    "Seed" | "" => {
                        let s = generate_seed(None);
                        generator.recent_seeds.push_front(s.to_string());
                        s
                    },
                    _ => {
                        if let Ok(n) = seed_string.parse::<u64>() {
                            generator.recent_seeds.push_front(seed_string.to_string());
                            n
                        } else {
                            let s = generate_seed(Some(seed_string));
                            generator.recent_seeds.push_front(seed_string.to_string());
                            s
                        }
                    },
                };
                if generator.time_check {
                    time_test(&generator.selected_stage, generator.test_count);
                } else {
                    generator.active_viewports.lock().push((seed_num, generator.selected_stage));
                }
            }
        });
    });
    ui.vertical(|ui| {
        for seed in &generator.recent_seeds {
            ui.label(seed);
        }
    });
    let ctx = ui.ctx().clone();
    for (seed, stage) in generator.active_viewports.lock().iter() {
        match stage {
            Stages::Stage1 => visualizer::show_stage_1(&ctx, *seed, Arc::clone(&generator.active_viewports)),
            Stages::Stage2 => visualizer::show_stage_2(&ctx, *seed, Arc::clone(&generator.active_viewports)),
            Stages::Stage3 => visualizer::show_stage_3(&ctx, *seed, Arc::clone(&generator.active_viewports)),
        }
    }
}