#![allow(dead_code)]

use std::hash::{Hash, Hasher};
use fnv::FnvHasher;
use rand::{SeedableRng, rngs::StdRng};

pub fn init_rng(input: Option<&str>) -> StdRng {
    let seed: u64 = match input {
        Some(s) => {
            let mut hasher: FnvHasher = FnvHasher::default();
            s.hash(&mut hasher);
            hasher.finish()
        }
        None => rand::random::<u64>()
    };
    StdRng::seed_from_u64(seed)
}

pub mod s1 {
    pub const DUNGEON_SIZE: usize = 18;
    pub const MID: usize = DUNGEON_SIZE / 2;
    pub const MAX_BOX_DIM: usize = 11;
    pub const BOX_COUNT: usize = 4;
    pub const ERODE_COUNT: usize = 5;

    pub const NULL:  u8 = 0b00000000;
    pub const ROOM:  u8 = 0b00010000;
    pub const NORTH: u8 = 0b00000001;
    pub const EAST:  u8 = 0b00000010;
    pub const SOUTH: u8 = 0b00000100;
    pub const WEST:  u8 = 0b00001000;
}