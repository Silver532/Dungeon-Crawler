use std::hash::{Hash, Hasher};
use fnv::FnvHasher;
use rand::{SeedableRng, rngs::StdRng};
use rand::distr::{Distribution, weighted::WeightedIndex};

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

pub fn get_room_shape(val: u8, rng: &mut StdRng) -> enums::Shape {
    use crate::helpers::shape_tables::*;
    let (shapes, weights): (&[enums::Shape], &[u8]) = match (val & 0x0F).count_ones() {
        0 => (&TABLE_0.0, &TABLE_1.1),
        1 => (&TABLE_1.0, &TABLE_1.1),
        2 => (&TABLE_2.0, &TABLE_2.1),
        3 => (&TABLE_3.0, &TABLE_3.1),
        4 => (&TABLE_4.0, &TABLE_4.1),
        _ => (&TABLE_0.0, &TABLE_0.1),
    };
    let dist: WeightedIndex<u8> = WeightedIndex::new(weights).unwrap();
    shapes[dist.sample(rng)]
}

pub fn get_room_theme(val: enums::Shape, rng: &mut StdRng) -> enums::Theme {
    use crate::helpers::theme_tables::*;
    use crate::helpers::enums::Shape::*;
    let (themes, weights): (&[enums::Theme], &[u8]) = match val {
        Null => (&TABLE_NULL.0, &TABLE_NULL.1),
        DeadEnd => (&TABLE_DEAD_END.0, &TABLE_DEAD_END.1),
        BossRoom => (&TABLE_BOSS_ROOM.0, &TABLE_BOSS_ROOM.1),
        SmallRoom => (&TABLE_SMALL_ROOM.0, &TABLE_SMALL_ROOM.1),
        LargeRoom => (&TABLE_LARGE_ROOM.0, &TABLE_LARGE_ROOM.1),
        Connection => (&TABLE_CONNECTION.0, &TABLE_CONNECTION.1),
        Corner => (&TABLE_CORNER.0, &TABLE_CORNER.1),
        Half => (&TABLE_HALF.0, &TABLE_HALF.1),
        SmallCircle => (&TABLE_SMALL_CIRCLE.0, &TABLE_SMALL_CIRCLE.1),
        LargeCircle => (&TABLE_LARGE_CIRCLE.0, &TABLE_LARGE_CIRCLE.1),
    };
    let dist: WeightedIndex<u8> = WeightedIndex::new(weights).unwrap();
    themes[dist.sample(rng)]
}

pub mod s1 {
    pub const DUNGEON_SIZE: usize = 18;
    pub const MID: usize = DUNGEON_SIZE / 2;
    pub const MAX_BOX_DIM: usize = 14;
    pub const BOX_COUNT: usize = 5;
    pub const ERODE_COUNT: usize = 5;

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

    impl From<u8> for Shape {
        fn from(val: u8) -> Self {
            match val {
                0 => Shape::Null,
                1 => Shape::DeadEnd,
                2 => Shape::BossRoom,
                3 => Shape::SmallRoom,
                4 => Shape::LargeRoom,
                5 => Shape::Connection,
                6 => Shape::Corner,
                7 => Shape::Half,
                8 => Shape::SmallCircle,
                9 => Shape::LargeCircle,
                _ => Shape::Null,
            }
        }
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

mod shape_tables {
    use crate::helpers::enums::Shape;

    pub const TABLE_0: ([Shape; 1], [u8; 1]) = ([Shape::Null], [100]);

    pub const TABLE_1: ([Shape; 5], [u8; 5]) = (
        [Shape::DeadEnd, Shape::BossRoom, Shape::SmallRoom, Shape::SmallCircle, Shape::LargeCircle],
        [30, 10, 35, 15, 10]
    );

    pub const TABLE_2: ([Shape; 5], [u8; 5]) = (
        [Shape::Connection, Shape::SmallRoom, Shape::LargeRoom, Shape::Corner, Shape::SmallCircle],
        [16, 27, 20, 22, 15]
    );

    pub const TABLE_3: ([Shape; 6], [u8; 6]) = (
        [Shape::Connection, Shape::SmallRoom, Shape::LargeRoom, Shape::Half, Shape::SmallCircle, Shape::LargeCircle],
        [17, 24, 22, 17, 12, 8]
    );

    pub const TABLE_4: ([Shape; 4], [u8; 4]) = (
        [Shape::Connection, Shape::SmallRoom, Shape::LargeRoom, Shape::LargeCircle],
        [15, 28, 40, 17]
    );
}

mod theme_tables {
    use crate::helpers::enums::Theme;

    pub const TABLE_NULL:         ([Theme; 1], [u8; 1]) = (
        [Theme::NULL],
        [100]
    );

    pub const TABLE_DEAD_END:     ([Theme; 5], [u8; 5]) = (
        [Theme::DeTRAPPED, Theme::DeTREASURE, Theme::DeHEALTHY, Theme::DeGUARDED, Theme::EMPTY],
        [20, 15, 10, 15, 40]
    );
    pub const TABLE_BOSS_ROOM:    ([Theme; 6], [u8; 6]) = (
        [Theme::BrHOARD, Theme::BrWIZARD, Theme::BrWEAK, Theme::BrSTRONG, Theme::BrGUARDED, Theme::BrDOUBLE],
        [20, 20, 20, 10, 20, 10]
    );
    pub const TABLE_SMALL_ROOM:   ([Theme; 7], [u8; 7]) = (
        [Theme::SrTRAPPED, Theme::SrTREASURE, Theme::SrGUARDED, Theme::SrCHAOS, Theme::SrBASIC, Theme::SrFLOODED, Theme::EMPTY],
        [20, 10, 15, 10, 25, 10, 10]
    );
    pub const TABLE_CONNECTION:   ([Theme; 5], [u8; 5]) = (
        [Theme::CnTRAPPED, Theme::CnGUARDED, Theme::CnBASIC, Theme::CnFLOODED, Theme::EMPTY],
        [20, 20, 25, 10, 25]
    );
    pub const TABLE_LARGE_ROOM:   ([Theme; 8], [u8; 8]) = (
        [Theme::LrTRAPPED, Theme::LrTREASURE, Theme::LrHEALTHY, Theme::LrGUARDED, Theme::LrCHAOS, Theme::LrBASIC, Theme::LrFLOODED, Theme::EMPTY],
        [20,  5,  5, 15, 10, 25, 10, 10]
    );
    pub const TABLE_CORNER:       ([Theme; 7], [u8; 7]) = (
        [Theme::CrTRAPPED, Theme::CrTREASURE, Theme::CrGUARDED, Theme::CrCHAOS, Theme::CrBASIC, Theme::CrFLOODED, Theme::EMPTY],
        [20, 10, 15, 10, 25, 10, 10]
    );
    pub const TABLE_HALF:         ([Theme; 7], [u8; 7]) = (
        [Theme::HrTRAPPED, Theme::HrTREASURE, Theme::HrGUARDED, Theme::HrCHAOS, Theme::HrBASIC, Theme::HrFLOODED, Theme::EMPTY],
        [20, 10, 15, 10, 25, 10, 10]
    );
    pub const TABLE_SMALL_CIRCLE: ([Theme; 7], [u8; 7]) = (
        [Theme::ScTRAPPED, Theme::ScTREASURE, Theme::ScGUARDED, Theme::ScCHAOS, Theme::ScBASIC, Theme::ScFLOODED, Theme::EMPTY],
        [20, 10, 15, 10, 25, 10, 10]
    );
    pub const TABLE_LARGE_CIRCLE: ([Theme; 8], [u8; 8]) = (
        [Theme::LcTRAPPED, Theme::LcTREASURE, Theme::LcHEALTHY, Theme::LcGUARDED, Theme::LcCHAOS, Theme::LcBASIC, Theme::LcFLOODED, Theme::EMPTY],
        [20,  5,  5, 15, 10, 25, 10, 10]
    );
}