use ndarray::Array2;
use rand::Rng;
use rand::rngs::StdRng;

use crate::helpers::{get_room_shape, get_room_theme};
use crate::helpers::enums::Shape;

fn get_entrance_room(exit_map: &Array2<u8>, rng: &mut StdRng) -> (usize, usize) {
    let possibilities: Vec<(usize, usize)> = exit_map.indexed_iter()
        .filter(|(_, v)| v.count_ones() == 2)
        .map(|((r, c), _)| (r, c))
        .collect();
    possibilities[rng.random_range(0..possibilities.len())]
}

fn get_shape_map(exit_map: &Array2<u8>, entrance_room: (usize, usize), rng: &mut StdRng) -> Array2<u8> {
    let rows: usize = exit_map.nrows();
    let cols: usize = exit_map.ncols();
    let mut shape_map: Array2<u8> = Array2::zeros((rows, cols));
    for ((row, col), val) in exit_map.indexed_iter() {
        if (row, col) == entrance_room {shape_map[[row, col]] = Shape::Entrance as u8; continue;}
        shape_map[[row,col]] = get_room_shape(*val, rng) as u8;
    }
    shape_map
}

fn get_theme_map(shape_map: &Array2<u8>, rng: &mut StdRng) -> Array2<u8> {
    let rows: usize = shape_map.nrows();
    let cols: usize = shape_map.ncols();
    let mut theme_map: Array2<u8> = Array2::zeros((rows, cols));
    for ((row, col), val) in shape_map.indexed_iter() {
        theme_map[[row,col]] = get_room_theme(Shape::from(*val), rng) as u8;
    }
    theme_map
}

pub fn plan_rooms(dungeon_map: &Array2<u8>, rng: &mut StdRng) -> (Array2<u8>, Array2<u8>) {
    let entrance_room: (usize, usize) = get_entrance_room(&dungeon_map, rng);
    let shape_map: Array2<u8> = get_shape_map(&dungeon_map, entrance_room, rng);
    let theme_map: Array2<u8> = get_theme_map(&shape_map, rng);
    (shape_map, theme_map)
}