use std::sync::Arc;

use eframe::egui::{self, mutex::Mutex};
use egui::ViewportBuilder;
use ndarray::Array2;
use generator::{run_stage_1};

pub fn show_stage_1(ctx: &egui::Context, seed: u64, active_viewports: Arc<Mutex<Vec<(u64, super::Stages)>>>) {
    let layout: Array2<u8> = run_stage_1(seed);
    let viewport_id = egui::ViewportId::from_hash_of(seed);
    let tile_size = 32.0;
    let rows = layout.nrows();
    let cols = layout.ncols();
    let height = rows as f32 * tile_size;
    let width = cols as f32 * tile_size;
    ctx.show_viewport_deferred(
        viewport_id,
        ViewportBuilder::default()
            .with_title(format!("Stage 1 - Seed: {}", seed))
            .with_inner_size([width, height]),
        move |ctx, _| {
            if ctx.input(|i| i.viewport().close_requested()) {
                active_viewports.lock().retain(|(s, _)| *s != seed)
            }
            egui::Area::new(egui::Id::new("visualizer_area"))
                .fixed_pos([0.0, 0.0])
                .show(ctx, |ui| {
                    let painter = ui.painter();
                    
                    for ((row, col), val) in layout.indexed_iter() {
                        let x = col as f32 * tile_size;
                        let y = row as f32 * tile_size;
                        let color = match val.count_ones() {
                            0 => egui::Color32::BLACK,
                            1 => egui::Color32::WHITE,
                            2 => egui::Color32::GREEN,
                            3 => egui::Color32::BLUE,
                            4 => egui::Color32::RED,
                            5 => egui::Color32::YELLOW,
                            _ => egui::Color32::from_rgb(255, 0, 255),
                        };
                        let rect = egui::Rect::from_min_size(
                            egui::pos2(x, y),
                            egui::vec2(tile_size, tile_size),
                        );
                        painter.rect_filled(rect, 0.0, color);
                    }
                });
        }
    );
}