mod stage_1;
//mod stage_2;
//mod stage_3;
mod helpers;

pub fn map_generator() {
    let mut rng = helpers::init_rng(None);
    
    let _layout = stage_1::generate_layout(&mut rng);
}