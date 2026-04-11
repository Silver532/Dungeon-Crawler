use rand::{Rng, rngs::StdRng};
use ndarray::{Array2, ArrayBase, Dim, OwnedRepr, s};

use crate::helpers::s1;

pub fn generate_layout(rng: &mut StdRng) -> ArrayBase<OwnedRepr<u8>, Dim<[usize; 2]>> {
    let mut dungeon_map: Array2<u8> = Array2::zeros((s1::DUNGEON_SIZE, s1::DUNGEON_SIZE));
    for _ in 0..s1::BOX_COUNT {
        let total = rng.random_range(5..s1::MAX_BOX_DIM);
        let height = rng.random_range(2..total-1);
        let width = total - height;

        let y_start = rng.random_range(1..s1::MID);
        let y_end = (y_start + height).max(s1::MID+1).min(s1::DUNGEON_SIZE-1);
        let x_start = rng.random_range(1..s1::MID);
        let x_end = (x_start + width).max(s1::MID+1).min(s1::DUNGEON_SIZE-1);

        dungeon_map.slice_mut(s![y_start..y_end, x_start..x_end]).fill(s1::ROOM)
    }
    return dungeon_map
}