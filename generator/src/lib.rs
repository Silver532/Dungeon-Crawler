use ndarray::{Array2};

mod stage_1;
//mod stage_2;
//mod stage_3;
pub mod helpers;

pub fn map_generator() -> Array2<u8> {
    let mut rng = helpers::init_rng(None);
    
    let layout = stage_1::generate_layout(&mut rng);
    return layout
}