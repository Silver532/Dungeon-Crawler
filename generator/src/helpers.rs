use rand::{SeedableRng, rngs::StdRng};
use fnv::FnvHasher;
use std::hash::{Hash, Hasher};

pub fn init_rng(input: Option<&str>) -> StdRng {
    let seed = match input {
        Some(s) => {
            let mut hasher = FnvHasher::default();
            s.hash(&mut hasher);
            hasher.finish()
        }
        None => rand::random::<u64>(), // random seed if none provided
    };
    StdRng::seed_from_u64(seed)
}

pub mod s1 {
    pub const DUNGEON_SIZE: usize = 14;
    pub const MID: usize = DUNGEON_SIZE / 2;
    pub const BOX_COUNT: usize = 3;
    pub const ERODE_COUNT: usize = 5;
}