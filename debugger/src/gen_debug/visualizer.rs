use std::sync::Arc;

use eframe::egui::{self, mutex::Mutex};
use egui::ViewportBuilder;
use generator::helpers::constants::ROOM_SIZE;
use generator::helpers::enums::{Shape, Theme, Tile};
use ndarray::Array2;

use super::Stages;
use super::CachedStage;

fn layout_color(val: u8) -> egui::Color32 {
    match val.count_ones() {
        0 => egui::Color32::BLACK,
        1 => egui::Color32::WHITE,
        2 => egui::Color32::GREEN,
        3 => egui::Color32::BLUE,
        4 => egui::Color32::RED,
        5 => egui::Color32::YELLOW,
        _ => egui::Color32::PURPLE,
    }
}

fn exits_string(val: u8) -> String {
    let exits: Vec<&str> = [("North", 1u8), ("East", 2), ("South", 4), ("West", 8)]
        .iter()
        .filter(|(_, bit)| val & bit != 0)
        .map(|(name, _)| *name)
        .collect();
    if exits.is_empty() {
        "None".to_string()
    } else {
        exits.join(", ")
    }
}

//Left for readability, this is not used enough for the change to matter
#[allow(clippy::too_many_arguments)]
fn show_viewport<C, H>(
    ctx: &egui::Context,
    seed: u64,
    stage: Stages,
    active_viewports: Arc<Mutex<Vec<(u64, Stages)>>>,
    title: String,
    rows: usize,
    cols: usize,
    tile_size: f32,
    text_height: f32,
    color_fn: C,
    hover_fn: H,
)
where
    C: Fn(usize, usize) -> egui::Color32 + Send + Sync + 'static,
    H: Fn(usize, usize) -> String + Send + Sync + 'static,
{
    let viewport_id = egui::ViewportId::from_hash_of((seed, stage as u8));
    let width: f32 = cols as f32 * tile_size;
    let height: f32 = rows as f32 * tile_size + text_height;

    ctx.show_viewport_deferred(
        viewport_id,
        ViewportBuilder::default()
            .with_title(title)
            .with_inner_size([width, height]),
        move |ctx, _| {
            if ctx.input(|i| i.viewport().close_requested()) {
                active_viewports
                    .lock()
                    .retain(|(s, st)| !(*s == seed && *st == stage));
            }

            egui::Area::new(egui::Id::new("visualizer_area"))
                .fixed_pos([0.0, 0.0])
                .show(ctx, |ui| {
                    let painter: &egui::Painter = ui.painter();

                    // Draw tiles
                    for row in 0..rows {
                        for col in 0..cols {
                            let color: egui::Color32 = color_fn(row, col);
                            let rect: egui::Rect = egui::Rect::from_min_size(
                                egui::pos2(col as f32 * tile_size, row as f32 * tile_size),
                                egui::vec2(tile_size, tile_size),
                            );
                            painter.rect_filled(rect, 0.0, color);
                        }
                    }

                    // Hover detection and info text
                    if text_height > 0.0 {
                        let hovered = ctx.input(|i| i.pointer.hover_pos()).and_then(|pos| {
                            let col: usize = (pos.x / tile_size) as usize;
                            let row: usize = (pos.y / tile_size) as usize;
                            if pos.y < rows as f32 * tile_size && row < rows && col < cols {
                                Some((row, col))
                            } else {
                                None
                            }
                        });

                        let text = match hovered {
                            Some((row, col)) => hover_fn(row, col),
                            None => String::new(),
                        };

                        painter.text(
                            egui::pos2(4.0, rows as f32 * tile_size + 2.0),
                            egui::Align2::LEFT_TOP,
                            text,
                            egui::FontId::monospace(18.0),
                            egui::Color32::WHITE,
                        );
                    }
                });
        },
    );
}

pub fn show_stage_1(
    ctx: &egui::Context,
    seed: u64,
    active_viewports: Arc<Mutex<Vec<(u64, Stages)>>>,
    data: &CachedStage,
) {
    let CachedStage::Stage1 { layout } = data else { return };

    let layout: Arc<Array2<u8>> = Arc::new(layout.clone());
    let layout_hover: Arc<Array2<u8>> = Arc::clone(&layout);

    let rows: usize = layout.nrows();
    let cols: usize = layout.ncols();

    show_viewport(
        ctx,
        seed,
        Stages::Stage1,
        active_viewports,
        format!("Stage 1 - Seed: {}", seed),
        rows,
        cols,
        48.0,
        48.0,
        move |row, col| layout_color(layout[[row, col]]),
        move |row, col| {
            let val = layout_hover[[row, col]];
            format!(
                "Tile ({}, {}) - Value: {}\nExits: {}",
                row, col, val,
                exits_string(val),
            )
        },
    );
}

pub fn show_stage_2(
    ctx: &egui::Context,
    seed: u64,
    active_viewports: Arc<Mutex<Vec<(u64, Stages)>>>,
    data: &CachedStage,
) {
    let CachedStage::Stage2 { layout, shape_map, theme_map } = data else { return };

    let layout: Arc<Array2<u8>> = Arc::new(layout.clone());
    let shape_map: Arc<Array2<u8>> = Arc::new(shape_map.clone());
    let theme_map: Arc<Array2<u8>> = Arc::new(theme_map.clone());

    let layout_hover: Arc<Array2<u8>> = Arc::clone(&layout);
    let shape_hover: Arc<Array2<u8>> = Arc::clone(&shape_map);
    let theme_hover: Arc<Array2<u8>> = Arc::clone(&theme_map);

    let rows: usize = layout.nrows();
    let cols: usize = layout.ncols();

    show_viewport(
        ctx,
        seed,
        Stages::Stage2,
        active_viewports,
        format!("Stage 2 - Seed: {}", seed),
        rows,
        cols,
        48.0,
        72.0,
        move |row, col| layout_color(layout[[row, col]]),
        move |row, col| {
            let val = layout_hover[[row, col]];
            format!(
                "Tile ({}, {}) - Value: {}\nExits: {}\nShape: {:?}  Theme: {:?}",
                row, col, val,
                exits_string(val),
                Shape::from(shape_hover[[row, col]]),
                Theme::from(theme_hover[[row, col]]),
            )
        },
    );
}

pub fn show_stage_3(
    ctx: &egui::Context,
    seed: u64,
    active_viewports: Arc<Mutex<Vec<(u64, Stages)>>>,
    data: &CachedStage,
) {
    let CachedStage::Stage3 { tilemap, theme_map } = data else { return };

    let tilemap: Arc<Array2<u8>> = Arc::new(tilemap.clone());
    let theme_map: Arc<Array2<u8>> = Arc::new(theme_map.clone());
    let tilemap_hover: Arc<Array2<u8>> = Arc::clone(&tilemap);
    let theme_hover: Arc<Array2<u8>> = Arc::clone(&theme_map);

    let rows: usize = tilemap.nrows();
    let cols: usize = tilemap.ncols();

    show_viewport(
        ctx,
        seed,
        Stages::Stage3,
        active_viewports,
        format!("Stage 3 - Seed: {}", seed),
        rows,
        cols,
        4.0,
        72.0,
        move |row, col| match Tile::from(tilemap[[row, col]]) {
            Tile::Wall           => egui::Color32::BLACK,
            Tile::Floor          => egui::Color32::WHITE,
            Tile::Water          => egui::Color32::from_rgb(26, 111, 204),
            Tile::Hole           => egui::Color32::from_rgb(128, 128, 128),
            Tile::HealingStation => egui::Color32::from_rgb(34, 170, 34),
            Tile::Shrine         => egui::Color32::from_rgb(192, 192, 192),
            Tile::Chest          => egui::Color32::from_rgb(139, 69, 19),
            Tile::LootPile       => egui::Color32::from_rgb(255, 215, 0),
            Tile::Trap           => egui::Color32::from_rgb(204, 0, 0),
            Tile::BossSpawner    => egui::Color32::from_rgb(255, 140, 0),
            Tile::MonsterSpawner => egui::Color32::from_rgb(255, 102, 0),
            Tile::Entrance       => egui::Color32::from_rgb(100, 100, 100),
            _                    => egui::Color32::BLACK,
        },
        move |row, col| {
            format!(
                "Tile ({}, {}) - {:?}\nTheme: {:?}",
                row, col,
                Tile::from(tilemap_hover[[row, col]]),
                Theme::from(theme_hover[[row/ROOM_SIZE, col/ROOM_SIZE]]),
            )
        },
    );
}