use rand::rngs::StdRng;
use rand::Rng;
use ndarray::Array2;

use crate::helpers::s1;

pub fn generate_layout(rng: &mut StdRng) {
    let mut dungeon_map: Array2<u8> = Array2::zeros((s1::DUNGEON_SIZE, s1::DUNGEON_SIZE));
    for _ in 0..s1::BOX_COUNT {
        let y_start = rng.random_range(1..s1::MID);
    }
}