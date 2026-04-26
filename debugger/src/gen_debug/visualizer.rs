use std::sync::Arc;

use eframe::egui::{self, mutex::Mutex};
use egui::ViewportBuilder;
use ndarray::Array2;
use generator::{helpers::enums::{Shape, Theme}, run_stage_1, run_stage_2};

pub fn show_stage_1(ctx: &egui::Context, seed: u64, active_viewports: Arc<Mutex<Vec<(u64, super::Stages)>>>) {
    let layout: Array2<u8> = run_stage_1(seed);
    let viewport_id: egui::ViewportId = egui::ViewportId::from_hash_of((seed, "stage1"));
    let tile_size: f32 = 48.0;
    let text_rows: u8 = 2;
    let text_height: f32 = (tile_size/2.0) * text_rows as f32;
    let rows: usize = layout.nrows();
    let cols: usize = layout.ncols();
    let height: f32 = rows as f32 * tile_size + text_height;
    let width: f32 = cols as f32 * tile_size;
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
                    let painter: &egui::Painter = ui.painter();
                    
                    for ((row, col), val) in layout.indexed_iter() {
                        let x: f32 = col as f32 * tile_size;
                        let y: f32 = row as f32 * tile_size;
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
                    let hovered_tile = ctx.input(|i| i.pointer.hover_pos()).and_then(|pos| {
                    let col = (pos.x / tile_size) as usize;
                    let row = (pos.y / tile_size) as usize;
                    if pos.y < rows as f32 * tile_size && row < rows && col < cols {
                        Some((row, col))
                    } else {
                        None
                    }
                });
                let text = match hovered_tile {
                    Some((row, col)) => {
                        let val = layout[[row, col]];
                        format!("Tile ({}, {}) - Value: {}\nExits: {}",
                            row, col,
                            val,
                            [("North", 1), ("East", 2), ("South", 4), ("West", 8)]
                                .iter()
                                .filter(|(_, bit)| val & bit != 0)
                                .map(|(name, _)| *name)
                                .collect::<Vec<_>>()
                                .join(", ")
                        )
                    }
                    None => String::new(),
                };
                painter.text(
                    egui::pos2(4.0, rows as f32 * tile_size + 2.0),
                    egui::Align2::LEFT_TOP,
                    text,
                    egui::FontId::monospace(text_height*1.5/4.0),
                    egui::Color32::WHITE,
                );
                });
        }
    );
}

pub fn show_stage_2(ctx: &egui::Context, seed: u64, active_viewports: Arc<Mutex<Vec<(u64, super::Stages)>>>) {
    let ((shape_map, theme_map), layout): ((Array2<u8>, Array2<u8>), Array2<u8>) = run_stage_2(seed);
    let viewport_id: egui::ViewportId = egui::ViewportId::from_hash_of((seed, "stage2"));
    let tile_size: f32 = 48.0;
    let text_rows: u8 = 3;
    let text_height: f32 = (tile_size/2.0) * text_rows as f32;
    let rows: usize = layout.nrows();
    let cols: usize = layout.ncols();
    let height: f32 = rows as f32 * tile_size + text_height;
    let width: f32 = cols as f32 * tile_size;
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
                    let hovered_tile = ctx.input(|i| i.pointer.hover_pos()).and_then(|pos| {
                    let col = (pos.x / tile_size) as usize;
                    let row = (pos.y / tile_size) as usize;
                    if pos.y < rows as f32 * tile_size && row < rows && col < cols {
                        Some((row, col))
                    } else {
                        None
                    }
                });
                let text = match hovered_tile {
                    Some((row, col)) => {
                        let val = layout[[row, col]];
                        format!("Tile ({}, {}) - Value: {}\nExits: {}\nShape: {:?} - Theme: {:?}",
                            row, col,
                            val,
                            [("North", 1), ("East", 2), ("South", 4), ("West", 8)]
                                .iter()
                                .filter(|(_, bit)| val & bit != 0)
                                .map(|(name, _)| *name)
                                .collect::<Vec<_>>()
                                .join(", "),
                            Shape::from(shape_map[[row, col]]),
                            Theme::from(theme_map[[row, col]]),
                        )
                    }
                    None => String::new(),
                };
                painter.text(
                    egui::pos2(4.0, rows as f32 * tile_size + 2.0),
                    egui::Align2::LEFT_TOP,
                    text,
                    egui::FontId::monospace(12.0),
                    egui::Color32::WHITE,
                );
                });
        }
    );
}