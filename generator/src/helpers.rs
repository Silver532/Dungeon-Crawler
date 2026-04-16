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
    pub const MAX_BOX_DIM: usize = 14;
    pub const BOX_COUNT: usize = 5;
    pub const ERODE_COUNT: usize = 5;

    pub const NULL:  u8 = 0b00000000;
    pub const ROOM:  u8 = 0b00010000;
    pub const NORTH: u8 = 0b00000001;
    pub const EAST:  u8 = 0b00000010;
    pub const SOUTH: u8 = 0b00000100;
    pub const WEST:  u8 = 0b00001000;
    pub const DIR_BITS: [u8; 4] = [1,2,4,8];
    pub const OPP_BITS: [u8; 4] = [4,8,1,2];
    pub const DY_DX: [[i8; 2]; 4] = [[-1, 0], [0, 1], [1, 0], [0, -1]];
    pub const MASK_TO_INDICES: [&'static [usize]; 16] = [
        &[],          // 0000
        &[0],         // 0001 N
        &[1],         // 0010 E
        &[0, 1],      // 0011 NE
        &[2],         // 0100 S
        &[0, 2],      // 0101 NS
        &[1, 2],      // 0110 ES
        &[0, 1, 2],   // 0111 NES
        &[3],         // 1000 W
        &[0, 3],      // 1001 NW
        &[1, 3],      // 1010 EW
        &[0, 1, 3],   // 1011 NEW
        &[2, 3],      // 1100 SW
        &[0, 2, 3],   // 1101 NSW
        &[1, 2, 3],   // 1110 ESW
        &[0, 1, 2, 3],// 1111 NESW
    ];
}

pub mod enums {
    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq)]
    pub enum Shape {
        Null = 0,
        DeadEnd,
        BossRoom,
        SmallRoom,
        LargeRoom,
        Connection,
        Corner,
        Half,
        SmallCircle,
        LargeCircle,
    }

    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq)]
    pub enum Theme {
        NULL = 0,
        EMPTY,
        ENTRANCE,

        DeTRAPPED,
        DeTREASURE,
        DeHEALTHY,
        DeGUARDED,

        BrHOARD,
        BrWIZARD,
        BrWEAK,
        BrSTRONG,
        BrGUARDED,
        BrDOUBLE,

        SrTRAPPED,
        SrTREASURE,
        SrGUARDED,
        SrCHAOS,
        SrBASIC,
        SrFLOODED,

        CnTRAPPED,
        CnGUARDED,
        CnBASIC,
        CnFLOODED,

        LrTRAPPED,
        LrTREASURE,
        LrHEALTHY,
        LrGUARDED,
        LrCHAOS,
        LrBASIC,
        LrFLOODED,

        CrTRAPPED,
        CrTREASURE,
        CrGUARDED,
        CrCHAOS,
        CrBASIC,
        CrFLOODED,

        HrTRAPPED,
        HrTREASURE,
        HrGUARDED,
        HrCHAOS,
        HrBASIC,
        HrFLOODED,

        ScTRAPPED,
        ScTREASURE,
        ScGUARDED,
        ScCHAOS,
        ScBASIC,
        ScFLOODED,
        
        LcTRAPPED,
        LcTREASURE,
        LcHEALTHY,
        LcGUARDED,
        LcCHAOS,
        LcBASIC,
        LcFLOODED,
    }

    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq)]
    pub enum Tile {
        Wall = 0,
        Floor = 1,
        Hole = 2,
        Water = 3,
        WaterPool = 4,
        Trap = 5,
        HealingStation = 6,
        Chest = 7,
        LootPile = 8,
        LootCluster = 9,
        MonsterSpawner = 10,
        BossSpawner = 11,
        Shrine = 12,
        Entrance = 13,
        PaintRed = 14,
        PaintBlue = 15,
        PaintGreen = 16,
    }
}