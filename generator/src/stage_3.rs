use ndarray::{Array2, ArrayViewMut2, Dimension, s};
use rand::rngs::StdRng;
use crate::helpers::{constants::*, enums::{Shape, Tile}};

fn build_room(mut view: ArrayViewMut2<u8>, val: u8, shape: Shape) {
    if val & NORTH != 0 {
        view.slice_mut(s![
            0..=HALF,
            HALF - 1..=HALF + 1
        ]).fill(Tile::Floor as u8);
    }

    if val & EAST != 0 {
        view.slice_mut(s![
            HALF - 1..=HALF + 1,
            HALF..ROOM_SIZE
        ]).fill(Tile::Floor as u8);
    }

    if val & SOUTH != 0 {
        view.slice_mut(s![
            HALF..ROOM_SIZE,
            HALF - 1..=HALF + 1
        ]).fill(Tile::Floor as u8);
    }

    if val & WEST != 0 {
        view.slice_mut(s![
            HALF - 1..=HALF + 1,
            0..=HALF
        ]).fill(Tile::Floor as u8);
    }
}

pub fn build_tilemap(layout: Array2<u8>, shapes: Array2<u8>, themes: &Array2<u8>, rng: &mut StdRng) -> Array2<u8> {
    let (height, width) = layout.raw_dim().into_pattern();
    let height_offset: usize = height * ROOM_SIZE;
    let width_offset: usize = width * ROOM_SIZE;
    let mut tilemap: Array2<u8> = Array2::zeros((height_offset, width_offset));
    // let mut _cache: Array2<u16> = Array2::zeros((height_offset, width_offset));


    for ((row, col), &val) in layout.indexed_iter() {
        let y0: usize = row * ROOM_SIZE;
        let x0: usize = col * ROOM_SIZE;
        if val != 0 {
            let shape: Shape = Shape::from(shapes[(row, col)]);
            let slice: ArrayViewMut2<u8> = tilemap.slice_mut(s![
                y0..y0 + ROOM_SIZE,
                x0..x0 + ROOM_SIZE
            ]);
            build_room(slice, val, shape);
        }
    }
    //Build Cache
    //Place Features

    tilemap
}