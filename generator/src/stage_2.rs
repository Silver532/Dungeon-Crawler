use ndarray::Array2;
use rand::rngs::StdRng;

use crate::helpers::get_room_shape;

fn get_shape_map(exit_map: &Array2<u8>, rng: &mut StdRng) -> Array2<u8> {
    let rows: usize = exit_map.nrows();
    let cols: usize = exit_map.ncols();
    let mut shape_map: Array2<u8> = Array2::zeros((rows, cols));
    for ((row, col), val) in exit_map.indexed_iter() {
        shape_map[[row,col]] = get_room_shape(*val, rng) as u8;
    }
    shape_map
}

fn get_theme_map(exit_map: &Array2<u8>, rng: &mut StdRng) -> Array2<u8> {
    let rows: usize = exit_map.nrows();
    let cols: usize = exit_map.ncols();
    let mut theme_map: Array2<u8> = Array2::zeros((rows, cols));
    for ((row, col), val) in exit_map.indexed_iter() {

    }
    theme_map
}

pub fn plan_rooms(dungeon_map: &Array2<u8>, rng: &mut StdRng) -> (Array2<u8>, Array2<u8>) {
    let shape_map: Array2<u8> = get_shape_map(&dungeon_map, rng);
    let theme_map: Array2<u8> = get_theme_map(&dungeon_map, rng);
    //Return Exit Map, Shape Map, Theme Map
    (shape_map, theme_map)
}