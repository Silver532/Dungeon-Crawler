use std::{collections::VecDeque, usize};

use rand::{Rng, rngs::StdRng, seq::SliceRandom};
use ndarray::{Array2, ArrayView2, s as slice};
use timing_macro::timeit;

use crate::helpers::s1;

#[timeit("Stage 1")]
fn place_boxes(tilemap: &mut Array2<u8>, rng: &mut StdRng) {
    debug_assert!(s1::MAX_BOX_DIM >= 12, "MAX_BOX_DIM too small for box generation ranges");
    for _ in 0..s1::BOX_COUNT {
        let total: usize = rng.random_range(8..s1::MAX_BOX_DIM);
        let height: usize = rng.random_range(2..total-3);
        let width: usize = total - height;
    
        let y_start: usize = rng.random_range(1..s1::MID+1);
        let y_end: usize = (y_start + height).max(s1::MID+1).min(s1::DUNGEON_SIZE-1);
        let x_start: usize = rng.random_range(1..s1::MID+1);
        let x_end: usize = (x_start + width).max(s1::MID+1).min(s1::DUNGEON_SIZE-1);
    
        tilemap.slice_mut(slice![y_start..y_end, x_start..x_end]).fill(s1::ROOM)
    }
}

#[timeit("Stage 1")]
fn count_neighbors(snapshot: &Array2<u8>, row: usize, col: usize) -> u8 {
    let mut count: u8 = 0;
    for y in -1..=1 {
        for x in -1..=1 {
            let row: isize = row as isize + y;
            let col: isize = col as isize + x;
            if row < 0 || col < 0
                || row >= snapshot.nrows() as isize
                || col >= snapshot.ncols() as isize
                {continue;}
            count += if snapshot[[row as usize, col as usize]] != 0 {1} else {0}
        }
    }
    count.saturating_sub(1)
}

#[timeit("Stage 1")]
fn erode_boxes(tilemap: &mut Array2<u8>, rng: &mut StdRng) {
    for _ in 0..s1::ERODE_COUNT {
        let snapshot: Array2<u8> = tilemap.clone();
        for ((row, col), val) in snapshot.indexed_iter() {
            if *val == 0 {continue;}
            let neighbors: u8 = count_neighbors(&snapshot, row, col);
            let erode_chance: u8 = match neighbors {
                0 => 100,
                1 => 90,
                2 | 3 => 35,
                4 | 5 => 15,
                6 | 7 => 3,
                8 => 1,
                _ => 0,
            };
            if rng.random_range(1..=100) <= erode_chance {
                tilemap[[row, col]] = 0;
            }
        }
    }
}

#[timeit("Stage 1")]
fn get_possible_connections(tilemap: &Array2<u8>) -> Array2<u8> {
    let tiles: Array2<u8> = tilemap.mapv(|v: u8| if v != 0 {1} else {0});
    let north: ArrayView2<u8>  = tiles.slice(slice![..-2, 1..-1]);
    let east: ArrayView2<u8>   = tiles.slice(slice![1..-1, 2..]);
    let south: ArrayView2<u8>  = tiles.slice(slice![2.., 1..-1]);
    let west: ArrayView2<u8>   = tiles.slice(slice![1..-1, ..-2]);
    let centre: ArrayView2<u8> = tiles.slice(slice![1..-1, 1..-1]);

    let connections: Array2<u8> = (&north
        | &east.mapv(|v: u8| v << 1)
        | &south.mapv(|v: u8| v << 2)
        | &west.mapv(|v: u8| v << 3))
        * &centre;
    return connections
}

#[timeit("Stage 1")]
fn connect_rooms(tilemap: &mut Array2<u8>, rng: &mut StdRng) {
    let connection_map: Array2<u8> = get_possible_connections(tilemap);

    let active: Vec<(usize, usize)> = tilemap.indexed_iter()
        .filter(|(_, v)| *v != &0)
        .map(|((r, c), _)| (r, c))
        .collect();

    for (row, col) in active {
        let mask: u8 = connection_map[[row-1, col-1]];
        let available: u8 = mask.count_ones() as u8;
        if available == 0 {
            tilemap[[row, col]] = 0;
            continue;
        }

        let connect_count: u8 = match rng.random::<f32>() {
            prob if prob >= 0.85 => 3,
            prob if prob >= 0.50 => 2,
            _ => 1
        }.min(available);

        let indices: &[usize] = s1::MASK_TO_INDICES[mask as usize];

        let chosen: Vec<usize> = if connect_count >= available {
            indices.to_vec()
        } else if connect_count == 1 {
            vec![indices[rng.random_range(0..indices.len())]]
        } else {
            let mut idx: Vec<usize> = indices.to_vec();
            idx.shuffle(rng);
            idx[..connect_count as usize].to_vec()
        };

        for i in chosen {
            let dy = s1::DY_DX[i][0] as isize;
            let dx = s1::DY_DX[i][1] as isize;
            let ny = (row as isize + dy) as usize;
            let nx = (col as isize + dx) as usize;
            tilemap[[row,col]] |= s1::DIR_BITS[i];
            tilemap[[ny,nx]] |= s1::OPP_BITS[i]
        }
    }
}

#[timeit("Stage 1")]
fn clear_rooms(tilemap: &mut Array2<u8>) {
    let (rows, cols) = tilemap.dim();

    let mut visited: Array2<bool> = Array2::<bool>::from_elem((rows, cols), false);

    let mut largest_group: Vec<(usize, usize)> = Vec::new();

    for row in 0..rows {
        for col in 0..cols {
            if tilemap[[row, col]] == 0 || visited[[row, col]] {
                continue;
            }

            let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
            let mut group: Vec<(usize, usize)> = Vec::new();

            visited[[row, col]] = true;
            queue.push_back((row, col));

            while let Some((r, c)) = queue.pop_front() {
                group.push((r, c));
                let val: u8 = tilemap[[r, c]];

                for i in 0..4 {
                    if val & (1 << i) == 0 {
                        continue;
                    }

                    let nr: usize = (r as isize + s1::DY_DX[i][0] as isize) as usize;
                    let nc: usize = (c as isize + s1::DY_DX[i][1] as isize) as usize;

                    if tilemap[[nr, nc]] != 0 && !visited[[nr, nc]] {
                        visited[[nr, nc]] = true;
                        queue.push_back((nr, nc));
                    }
                }
            }

            if group.len() > largest_group.len() {
                largest_group = group;
            }
        }
    }

    let mut keep: Array2<bool> = Array2::<bool>::from_elem((rows, cols), false);
    for (r, c) in largest_group {
        keep[[r, c]] = true;
    }

    for row in 0..rows {
        for col in 0..cols {
            if !keep[[row, col]] {
                tilemap[[row, col]] = 0;
            }
        }
    }
}

#[timeit("Stage 1")]
fn prep_entrance(tilemap: &mut Array2<u8>, rng: &mut StdRng) {
    let one_exit_list: Vec<(usize, usize)> = tilemap.indexed_iter()
        .filter(|(_, v)| v.count_ones() == 2)
        .map(|((r, c), _)| (r, c))
        .collect();
    if !one_exit_list.is_empty() {return;}

    let tiles: Array2<u8> = tilemap.mapv(|v: u8| if v != 0 { 1 } else { 0 });
    let empty: Array2<u8> = 1 - &tiles;
    let north: ArrayView2<u8> = tiles.slice(slice![..-2, 1..-1]);
    let east: ArrayView2<u8>  = tiles.slice(slice![1..-1, 2..]);
    let south: ArrayView2<u8> = tiles.slice(slice![2..,  1..-1]);
    let west: ArrayView2<u8>  = tiles.slice(slice![1..-1, ..-2]);
    let centre: ArrayView2<u8> = empty.slice(slice![1..-1, 1..-1]);

    let candidates: Array2<u8> = (&north | &east | &south | &west) * &centre;
    
    let indices: Vec<(usize, usize)> = candidates.indexed_iter()
        .filter(|(_, v)| **v != 0)
        .map(|((r, c), _)| (r + 1, c + 1))
        .collect();

    if indices.is_empty() {
        tilemap[[s1::MID, s1::MID]]     = s1::ROOM | s1::SOUTH;
        tilemap[[s1::MID + 1, s1::MID]] = s1::ROOM | s1::NORTH;
        return;
    }

    let (row, col) = indices[rng.random_range(0..indices.len())];
    let (rows, cols) = tilemap.dim();
    for (i, [dy, dx]) in s1::DY_DX.iter().enumerate() {
        let nr: i8 = row as i8 + dy;
        let nc: i8 = col as i8 + dx;
        if nr < 0 || nc < 0 { continue; }
        let (nr, nc) = (nr as usize, nc as usize);
        if nr >= rows || nc >= cols { continue; }
        if tilemap[[nr, nc]] != 0 {
            tilemap[[row, col]] = s1::ROOM | s1::OPP_BITS[i];
            tilemap[[nr, nc]] |= s1::DIR_BITS[i];
            return;
        }
    }
}

#[timeit("Stage 1")]
fn trim_tilemap(tilemap: Array2<u8>) -> Array2<u8> {
    let mut min_row: usize = usize::MAX;
    let mut max_row: usize = 0;
    let mut min_col: usize = usize::MAX;
    let mut max_col: usize = 0;
    for ((row, col), &val) in tilemap.indexed_iter() {
        if val != 0 {
            min_row = min_row.min(row);
            max_row = max_row.max(row);
            min_col = min_col.min(col);
            max_col = max_col.max(col);
        }
    }
    tilemap.slice(slice![min_row..=max_row, min_col..=max_col]).to_owned()
}

#[timeit("Stage 1")]
pub fn generate_layout(rng: &mut StdRng) -> Array2<u8> {
    let mut dungeon_map: Array2<u8> = Array2::zeros((s1::DUNGEON_SIZE, s1::DUNGEON_SIZE));
    place_boxes(&mut dungeon_map, rng);
    erode_boxes(&mut dungeon_map, rng);
    connect_rooms(&mut dungeon_map, rng);
    clear_rooms(&mut dungeon_map);
    prep_entrance(&mut dungeon_map, rng);
    let dungeon_map: Array2<u8> = trim_tilemap(dungeon_map);
    dungeon_map
}