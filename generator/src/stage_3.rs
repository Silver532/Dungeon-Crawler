use ndarray::{Array2, ArrayViewMut2, Dimension, s};
use rand::{Rng, rngs::StdRng};
use crate::helpers::{constants::*, enums::{Shape, Tile}};

fn build_room(mut view: ArrayViewMut2<u8>, val: u8, shape: Shape, rng: &mut StdRng) {
    let north: bool = (val & NORTH) != 0;
    let east:  bool = (val & EAST)  != 0;
    let south: bool = (val & SOUTH) != 0;
    let west:  bool = (val & WEST)  != 0;

    if north {
        view.slice_mut(s![
            0..=HALF + 1,
            HALF - 1..=HALF + 1
        ]).fill(Tile::Floor as u8);
    }

    if east {
        view.slice_mut(s![
            HALF - 1..=HALF + 1,
            HALF - 1..ROOM_SIZE
        ]).fill(Tile::Floor as u8);
    }

    if south {
        view.slice_mut(s![
            HALF - 1..ROOM_SIZE,
            HALF - 1..=HALF + 1
        ]).fill(Tile::Floor as u8);
    }

    if west {
        view.slice_mut(s![
            HALF - 1..=HALF + 1,
            0..=HALF + 1
        ]).fill(Tile::Floor as u8);
    }

    match shape {
        Shape::DeadEnd => {
            let length: usize = rng.random_range(2..=5);
            view.slice_mut(s![
                HALF - length..=HALF + length,
                HALF - length..=HALF + length
            ]).fill(Tile::Wall as u8)
        }
        Shape::BossRoom => {
            view.slice_mut(s![
                1..-1,
                1..-1
            ]).fill(Tile::Floor as u8);
            if rng.random_bool(0.34) {
                view[[3, 3]]   = Tile::Wall as u8;
                view[[13, 3]]  = Tile::Wall as u8;
                view[[3, 13]]  = Tile::Wall as u8;
                view[[13, 13]] = Tile::Wall as u8;
            }
        }
        Shape::SmallRoom => {
            view.slice_mut(s![
                HALF-3..=HALF+3,
                HALF-3..=HALF+3
            ]).fill(Tile::Floor as u8);
        }
        Shape::Connection => {}
        Shape::LargeRoom => {
            view.slice_mut(s![
                HALF-6..=HALF+6,
                HALF-6..=HALF+6
            ]).fill(Tile::Floor as u8);
        }
        Shape::Corner => {
            let lo = 1..=HALF + 1;
            let hi = (HALF - 1) as isize..-1;

            match (north, south, east, west) {
                (true, _, true, _) => view.slice_mut(s![lo.clone(), hi.clone()]),
                (_, true, true, _) => view.slice_mut(s![hi.clone(), hi.clone()]),
                (true, _, _, true) => view.slice_mut(s![lo.clone(), lo.clone()]),
                (_, true, _, true) => view.slice_mut(s![hi.clone(), lo.clone()]),
                _ => return
            }.fill(Tile::Floor as u8);
        }
        Shape::Half => {
            match (north, south, east, west) {
                (true, true, true, _) => view.slice_mut(s![1..-1, HALF as isize..-1]),
                (true, true, _, true) => view.slice_mut(s![1..-1, 0..=HALF]),
                (true, _, true, true) => view.slice_mut(s![1..=HALF, 0..-1]),
                (_, true, true, true) => view.slice_mut(s![HALF as isize..-1, 1..-1]),
                _ => return
            }.fill(Tile::Floor as u8);
        }
        Shape::SmallCircle => {
            const MASK: [u16; 9] = [
                0b001111100,
                0b011111110,
                0b111111111,
                0b111111111,
                0b111111111,
                0b111111111,
                0b111111111,
                0b011111110,
                0b001111100,
            ];
            for dy in 0..9 {
                for dx in 0..9 {
                    if MASK[dy] & (1 << (8 - dx)) != 0 {
                        view[[HALF - 4 + dy, HALF - 4 + dx]] = Tile::Floor as u8;
                    }
                }
            }
        }
        Shape::LargeCircle => {
            const MASK: [u16; 15] = [
                0b000001111100000,
                0b000111111111000,
                0b001111111111100,
                0b011111111111110,
                0b011111111111110,
                0b111111111111111,
                0b111111111111111,
                0b111111111111111,
                0b111111111111111,
                0b111111111111111,
                0b011111111111110,
                0b011111111111110,
                0b001111111111100,
                0b000111111111000,
                0b000001111100000,
            ];
            for dy in 0..15 {
                for dx in 0..15 {
                    if MASK[dy] & (1 << (14 - dx)) != 0 {
                        view[[HALF - 7 + dy, HALF - 7 + dx]] = Tile::Floor as u8;
                    }
                }
            }
        }
        _ => {}
    }
}

pub fn build_tilemap(layout: Array2<u8>, shapes: Array2<u8>, themes: &Array2<u8>, rng: &mut StdRng) -> Array2<u8> {
    let (height, width) = layout.raw_dim().into_pattern();
    let height_offset: usize = height * ROOM_SIZE;
    let width_offset: usize = width * ROOM_SIZE;
    let mut tilemap: Array2<u8> = Array2::zeros((height_offset, width_offset));
    let mut cache: Array2<u16> = Array2::zeros((height_offset, width_offset));

    for ((row, col), &val) in layout.indexed_iter() {
        let y0: usize = row * ROOM_SIZE;
        let x0: usize = col * ROOM_SIZE;
        if val != 0 {
            let shape: Shape = Shape::from(shapes[(row, col)]);
            let slice: ArrayViewMut2<u8> = tilemap.slice_mut(s![
                y0..y0 + ROOM_SIZE,
                x0..x0 + ROOM_SIZE
            ]);
            build_room(slice, val, shape, rng);
        }
    }
    //Build Cache
    //Place Features

    tilemap
}