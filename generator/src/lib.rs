use ndarray::{ArrayBase, Dim, OwnedRepr};

mod stage_1;
//mod stage_2;
//mod stage_3;
mod helpers;

pub fn map_generator() -> ArrayBase<OwnedRepr<u8>, Dim<[usize; 2]>> {
    let mut rng = helpers::init_rng(None);
    
    let layout = stage_1::generate_layout(&mut rng);
    return layout
}