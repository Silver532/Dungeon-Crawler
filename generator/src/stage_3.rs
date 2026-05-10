use ndarray::Array2;
use rand::rngs::StdRng;
use crate::helpers::s1::DUNGEON_SIZE;

pub fn build_tilemap(layout: Array2<u8>, shapes: Array2<u8>, themes: &Array2<u8>, rng: &mut StdRng) -> Array2<u8> {
    let (height, width) = layout.raw_dim().into_pattern();
    let tilemap: Array2<u8> = Array2::zeros((height * DUNGEON_SIZE, width * DUNGEON_SIZE));
    let cache: Array2<u16> = Array2::zeros((height * DUNGEON_SIZE, width * DUNGEON_SIZE));
    //Initialize Arrays
    //Build Rooms
    //Build Cache
    //Place Features
    
    layout
}