use ndarray::{Array2};
use macroquad::prelude::*;

use generator::map_generator;
use generator::helpers::s1;

const TILE_WIDTH: f32 = 30.0;
const INFO_BAR_HEIGHT: f32 = 30.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Visualizer".to_owned(),
        window_width: 500,
        window_height: 500,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let layout: Array2<u8> = map_generator();
    let width = layout.ncols() as f32 * TILE_WIDTH;
    let height = layout.nrows() as f32 * TILE_WIDTH + INFO_BAR_HEIGHT;

    request_new_screen_size(width, height);
    next_frame().await;
    loop {
        clear_background(WHITE);
        for ((row, col), val) in layout.indexed_iter() {
            let x: f32 = col as f32 * TILE_WIDTH;
            let y: f32 = row as f32 * TILE_WIDTH;

            let colour = match val.count_ones() {
                0 => Color::from_hex(0x000000),
                1 => Color::from_hex(0xFFFFFF),
                2 => Color::from_hex(0x00FF00),
                3 => Color::from_hex(0x0000FF),
                4 => Color::from_hex(0xFF0000),
                5 => Color::from_hex(0xFFFF00),
                _ => PINK
            };

            draw_rectangle(x, y, TILE_WIDTH, TILE_WIDTH, colour);

        }
        let (mx, my) = mouse_position();
        let hover_col = (mx / TILE_WIDTH) as usize;
        let hover_row = (my / TILE_WIDTH) as usize;

        if hover_row < layout.nrows() && hover_col < layout.ncols() {
            let val = layout[[hover_row, hover_col]];
            let exits = format!(
                "{}{}{}{}",
                if val & s1::NORTH != 0 { "North "} else { "" },
                if val & s1::EAST  != 0 { "East " } else { "" },
                if val & s1::SOUTH != 0 { "South "} else { "" },
                if val & s1::WEST  != 0 { "West"  } else { "" },
            );
            let info = format!("({:-2},{:-2}) val:{:2} {}", hover_row, hover_col, val.saturating_sub(16), exits);
            draw_text(&info, 10.0, screen_height() - (INFO_BAR_HEIGHT/2.0), 20.0, BLACK);
        }

        next_frame().await
    }
}