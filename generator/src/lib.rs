use ndarray::Array2;

mod stage_1;
mod stage_2;
//mod stage_3;
pub mod helpers;

pub fn run_stage_1() -> Array2<u8> {
    let mut rng = helpers::init_rng(None);
    stage_1::generate_layout(&mut rng)
}

pub fn run_stage_2() -> (Array2<u8>, Array2<u8>) {
    let mut rng = helpers::init_rng(None);
    let layout: Array2<u8> = stage_1::generate_layout(&mut rng);
    stage_2::plan_rooms(&layout, &mut rng)
}

pub fn map_generator() -> (Array2<u8>, Array2<u8>) {
    let mut rng = helpers::init_rng(None);
    
    let layout: Array2<u8> = stage_1::generate_layout(&mut rng);
    let (shape_map, theme_map) = stage_2::plan_rooms(&layout, &mut rng);
    (shape_map, theme_map)
}