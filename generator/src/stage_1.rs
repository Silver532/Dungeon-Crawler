use std::{collections::{HashSet, VecDeque}, usize};

use rand::{Rng, rngs::StdRng, seq::SliceRandom};
use ndarray::{Array2, ArrayView2, s as slice};

use crate::helpers::s1;

fn place_boxes(tilemap: &mut Array2<u8>, rng: &mut StdRng) {
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

fn get_possible_connections(tilemap: &Array2<u8>) -> Array2<u8> {
    let tiles: Array2<u8> = tilemap.mapv(|v: u8| if v != 0 {1} else {0});
    let north: ArrayView2<u8>  = tiles.slice(slice![..-2, 1..-1]);
    let east: ArrayView2<u8>   = tiles.slice(slice![1..-1, 2..]);
    let south: ArrayView2<u8>  = tiles.slice(slice![2.., 1..-1]);
    let west: ArrayView2<u8>   = tiles.slice(slice![1..-1, ..-2]);
    let centre: ArrayView2<u8> = tiles.slice(slice![1..-1, 1..-1]);

    let connections: Array2<u8> = (&north | &east.mapv(|v: u8| v << 1) | &south.mapv(|v: u8| v << 2) | &west.mapv(|v: u8| v << 3)) * &centre;
    return connections
}

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
            rand if rand >= 0.85 => 3,
            rand if rand >= 0.50 => 2,
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

fn clear_rooms(tilemap: &mut Array2<u8>) {
    let active: HashSet<(usize, usize)> = tilemap.indexed_iter()
        .filter(|(_, v)| **v != 0)
        .map(|((r, c), _)| (r, c))
        .collect();
    let mut unvisited: HashSet<(usize, usize)> = active.clone();
    let mut groups: Vec<HashSet<(usize, usize)>> = Vec::new();
    while !unvisited.is_empty() {
        let start: (usize, usize) = *unvisited.iter().next().unwrap();
        let mut group: HashSet<(usize,usize)> = HashSet::new();
        let mut visited: HashSet<(usize,usize)> = HashSet::from([start]);
        let mut queue: VecDeque<(usize,usize)> = VecDeque::from([start]);
        while let Some(tile) = queue.pop_front() {
            group.insert(tile);
            let row: usize = tile.0;
            let col: usize = tile.1;
            let val: u8 = tilemap[[row, col]];
            for i in 0..4 {
                if val & (1 << i) != 0 {
                    let ny: isize = row as isize + s1::DY_DX[i][0] as isize;
                    let nx: isize = col as isize + s1::DY_DX[i][1] as isize;
                    if ny >= 0 && nx >= 0 && ny < tilemap.nrows() as isize && nx < tilemap.ncols() as isize {
                        let neighbor = (ny as usize, nx as usize);
                        if !visited.contains(&neighbor) {
                            visited.insert(neighbor);
                            queue.push_back(neighbor);
                        }
                    }
                }
            }
        }
        groups.push(group);
        unvisited.retain(|t| !visited.contains(t));
    }
    if !groups.is_empty() {
        if let Some(largest) = groups.iter().max_by_key(|g| g.len()) {
            for index in active {
                if !largest.contains(&index) {
                    tilemap[[index.0, index.1]] = 0;
                }
            }
        }
    }
}

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
    tilemap.slice(slice![min_row..=max_row, min_col..max_col]).to_owned()
}

pub fn generate_layout(rng: &mut StdRng) -> Array2<u8> {
    let mut dungeon_map: Array2<u8> = Array2::zeros((s1::DUNGEON_SIZE, s1::DUNGEON_SIZE));
    place_boxes(&mut dungeon_map, rng);
    erode_boxes(&mut dungeon_map, rng);
    connect_rooms(&mut dungeon_map, rng);
    clear_rooms(&mut dungeon_map);
    let dungeon_map: Array2<u8> = trim_tilemap(dungeon_map);
    dungeon_map
}