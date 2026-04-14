use rand::{Rng, rngs::StdRng};
use ndarray::{Array2, s as slice};

use crate::helpers::s1;

fn place_boxes(tilemap: &mut Array2<u8>, rng: &mut StdRng) {
    for _ in 0..s1::BOX_COUNT {
        let total: usize = rng.random_range(7..s1::MAX_BOX_DIM);
        let height: usize = rng.random_range(2..total-2);
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
                1 => 80,
                2 | 3 => 40,
                4 | 5 => 15,
                6 | 7 => 5,
                8 => 1,
                _ => 0,
            };
            if rng.random_range(1..=100) <= erode_chance {
                tilemap[[row, col]] = 0;
            }
        }
    }
}

pub fn generate_layout(rng: &mut StdRng) -> Array2<u8> {
    let mut dungeon_map: Array2<u8> = Array2::zeros((s1::DUNGEON_SIZE, s1::DUNGEON_SIZE));
    place_boxes(&mut dungeon_map, rng);
    erode_boxes(&mut dungeon_map, rng);
    //Connect Rooms
    //Clear Pass
    //Trim Dungeon Map
    dungeon_map
}