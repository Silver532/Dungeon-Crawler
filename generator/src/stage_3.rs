#[allow(clippy::reversed_empty_ranges)]
//Uses ndarray negative indexing, these are not reversed ranges
//Clippy does not understand this and falsely flags them

use ndarray::{Array2, ArrayViewMut2, Dimension, s};
use rand::{Rng, rngs::StdRng, seq::IndexedRandom};
use timing_macro::timeit;
use crate::helpers::{constants::*, enums::{Shape, Theme, Tile}, feature_placement::{self, ScanParams}};

#[timeit("Stage 3")]
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
            view.slice_mut(s![HALF-4..=HALF+4, HALF-2..=HALF+2]).fill(Tile::Floor as u8);
            view.slice_mut(s![HALF-3..=HALF+3, HALF-3..=HALF+3]).fill(Tile::Floor as u8);
            view.slice_mut(s![HALF-2..=HALF+2, HALF-4..=HALF+4]).fill(Tile::Floor as u8);
        }
        Shape::LargeCircle => {
            view.slice_mut(s![HALF-7..=HALF+7, HALF-3..=HALF+3]).fill(Tile::Floor as u8);
            view.slice_mut(s![HALF-6..=HALF+6, HALF-4..=HALF+4]).fill(Tile::Floor as u8);
            view.slice_mut(s![HALF-5..=HALF+5, HALF-5..=HALF+5]).fill(Tile::Floor as u8);
            view.slice_mut(s![HALF-4..=HALF+4, HALF-6..=HALF+6]).fill(Tile::Floor as u8);
            view.slice_mut(s![HALF-3..=HALF+3, HALF-7..=HALF+7]).fill(Tile::Floor as u8);
        }
        Shape::Entrance => {
            view.slice_mut(s![
                HALF - 2..=HALF + 2,
                HALF - 2..=HALF + 2
            ]).fill(Tile::Floor as u8);
            match (north, south, east, west) {
                (true, _, _, _) => {
                    view[[HALF+3,HALF]] = Tile::Entrance as u8;
                    view[[HALF+2,HALF+2]] = Tile::Wall as u8;
                    view[[HALF+2,HALF-2]] = Tile::Wall as u8;
                }
                (_, true, _, _) => {
                    view[[HALF-3,HALF]] = Tile::Entrance as u8;
                    view[[HALF-2,HALF+2]] = Tile::Wall as u8;
                    view[[HALF-2,HALF-2]] = Tile::Wall as u8;
                }
                (_, _, true, _) => {
                    view[[HALF,HALF-3]] = Tile::Entrance as u8;
                    view[[HALF+2,HALF-2]] = Tile::Wall as u8;
                    view[[HALF-2,HALF-2]] = Tile::Wall as u8;
                }
                (_, _, _, true) => {
                    view[[HALF,HALF+3]] = Tile::Entrance as u8;
                    view[[HALF+2,HALF+2]] = Tile::Wall as u8;
                    view[[HALF-2,HALF+2]] = Tile::Wall as u8;
                }
                _ => return
            }
        }
        _ => {}
    }
}

#[timeit("Stage 3")]
fn scan_room(tilemap: &Array2<u8>, cache: &Array2<u16>, params: &ScanParams, y0: usize, x0: usize, candidates: &mut Vec<(usize, usize)>, biased: &mut Vec<(usize, usize)>) {
    candidates.clear();
    biased.clear();
    
    let place_on: u16 = params.place_on.unwrap_or(FLOOR_MASK);
    let tilemap_raw: &[u8] = tilemap.as_slice().unwrap();
    let cache_raw: &[u16] = cache.as_slice().unwrap();
    let width: usize = tilemap.ncols();

    for row in 0..ROOM_SIZE {
        let abs_row: usize = y0 + row;
        for col in 0..ROOM_SIZE {
            let abs_col: usize = x0 + col;
            let idx: usize = abs_row * width + abs_col;
            let tile_bit: u16 = 1 << tilemap_raw[idx];
            if tile_bit & place_on == 0 { continue }
            let neighbors: u16 = cache_raw[idx];
            if params.require != 0 && neighbors & params.require == 0 { continue }
            if params.block != 0 && neighbors & params.block != 0 { continue }
            if params.bias != 0 && neighbors & params.bias != 0 {
                biased.push((abs_row, abs_col))
            }
            candidates.push((abs_row, abs_col))
        }
    }

    if !biased.is_empty() {
        candidates.extend_from_slice(biased);
        candidates.extend_from_slice(biased);
        candidates.extend_from_slice(biased);
    }
}

#[timeit("Stage 3")]
fn place_features(mut tilemap: Array2<u8>, mut cache: Array2<u16>, theme_map: &Array2<u8>, rng: &mut StdRng) -> Array2<u8> {
    let (height, width) = tilemap.dim();
    let mut candidates: Vec<(usize, usize)> = Vec::new();
    let mut biases: Vec<(usize, usize)> = Vec::new();
    for ((row, col), &val) in theme_map.indexed_iter() {
        let theme: Theme = Theme::from(val);
        if matches!(theme, Theme::Null | Theme::Empty | Theme::Entrance) { continue }

        let y0: usize = row * ROOM_SIZE;
        let x0: usize = col * ROOM_SIZE;
        let counts: [u8; 17] = feature_placement::map(theme, rng);

        for feature in FEATURE_ORDER {
            let count: u8 = counts[feature as usize];
            if count == 0 { continue }

            let params: &ScanParams = &feature_placement::SCAN_PARAMS[feature as usize];

            scan_room(&tilemap, &cache, params, y0, x0, &mut candidates, &mut biases);
            if candidates.is_empty() { continue }

            let quota: usize = count.min(candidates.len() as u8) as usize;
            let chosen = candidates.choose_multiple(rng, quota);
            for &(r, c) in chosen {
                let tile_val: u8 = match feature {
                    Tile::WaterPool => Tile::Water as u8,
                    Tile::LootCluster => Tile::LootPile as u8,
                    _ => feature as u8,
                };
                tilemap[[r, c]] = tile_val;
                let bit: u16 = 1 << tile_val;

                if r > 0           {cache[[r - 1, c]] |= bit;}
                if r < height - 1  {cache[[r + 1, c]] |= bit;}
                if c > 0           {cache[[r, c - 1]] |= bit;}
                if c < width - 1   {cache[[r, c + 1]] |= bit;}
                cache[[r, c]] |= bit
            }
        }
    }
    tilemap
}

#[timeit("Stage 3")]
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
            build_room(slice, val, shape, rng)
        }
    }

    let tilemap_raw: &[u8] = tilemap.as_slice().unwrap();
    let cache_raw: &mut [u16] = cache.as_slice_mut().unwrap();

    for row in 1..height_offset - 1 {
        for col in 1..width_offset - 1 {
            let idx: usize = row * width_offset + col;
            cache_raw[idx] =
                (1 << tilemap_raw[idx - width_offset]) |
                (1 << tilemap_raw[idx + width_offset]) |
                (1 << tilemap_raw[idx - 1]) |
                (1 << tilemap_raw[idx + 1])
        }
    }

    for row in 0..height_offset {
        for col in 0..width_offset {
            if row > 0 && row < height_offset - 1 && col > 0 && col < width_offset - 1 { continue }
            let idx: usize = row * width_offset + col;
            let mut mask: u16 = 0;
            if row > 0               {mask |= 1 << tilemap_raw[idx - width_offset]}
            if row < height_offset-1 {mask |= 1 << tilemap_raw[idx + width_offset]}
            if col > 0               {mask |= 1 << tilemap_raw[idx - 1]}
            if col < width_offset-1  {mask |= 1 << tilemap_raw[idx + 1]}
            cache_raw[idx] = mask
        }
    }

    place_features(tilemap, cache, themes, rng)
}