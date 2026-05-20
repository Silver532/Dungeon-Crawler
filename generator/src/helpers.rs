use rand::{SeedableRng, rngs::StdRng};
use rand::distr::{Distribution, weighted::WeightedIndex};

pub fn init_rng(input: u64) -> StdRng {
    StdRng::seed_from_u64(input)
}

pub fn get_room_shape(val: u8, rng: &mut StdRng) -> enums::Shape {
    use crate::helpers::shape_tables::*;
    let (shapes, weights): (&[enums::Shape], &[u8]) = match (val & 0x0F).count_ones() {
        0 => (&TABLE_0.0, &TABLE_0.1),
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
        Entrance => (&TABLE_ENTRANCE.0, &TABLE_ENTRANCE.1),
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
    pub const MASK_TO_INDICES: [&[usize]; 16] = [
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

pub mod constants {
    use crate::helpers::enums::Tile;

    pub const ROOM_SIZE: usize = 17;
    pub const HALF: usize = ROOM_SIZE/2;
    pub const NORTH: u8 = 0b00001;
    pub const EAST:  u8 = 0b00010;
    pub const SOUTH: u8 = 0b00100;
    pub const WEST:  u8 = 0b01000;
    pub const FEATURE_ORDER: [Tile; 11] = [
        Tile::Water,
        Tile::WaterPool,
        Tile::Hole,
        Tile::HealingStation,
        Tile::Shrine,
        Tile::Chest,
        Tile::LootPile,
        Tile::LootCluster,
        Tile::Trap,
        Tile::BossSpawner,
        Tile::MonsterSpawner
    ];
    pub const FLOOR_MASK: u16 = 1 << Tile::Floor as u8;
}

pub mod enums {
    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Debug)]
    pub enum Shape {
        Null = 0,
        Entrance,
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
                1 => Shape::Entrance,
                2 => Shape::DeadEnd,
                3 => Shape::BossRoom,
                4 => Shape::SmallRoom,
                5 => Shape::LargeRoom,
                6 => Shape::Connection,
                7 => Shape::Corner,
                8 => Shape::Half,
                9 => Shape::SmallCircle,
                10 => Shape::LargeCircle,
                _ => Shape::Null,
            }
        }
    }

    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Debug)]
    pub enum Theme {
        Null = 0,
        Empty,
        Entrance,

        DeTrapped,
        DeTreasure,
        DeHealthy,
        DeGuarded,

        BrHoard,
        BrWizard,
        BrWeak,
        BrStrong,
        BrGuarded,
        BrDouble,

        SrTrapped,
        SrTreasure,
        SrGuarded,
        SrChaos,
        SrBasic,
        SrFlooded,

        CnTrapped,
        CnGuarded,
        CnBasic,
        CnFlooded,

        LrTrapped,
        LrTreasure,
        LrHealthy,
        LrGuarded,
        LrChaos,
        LrBasic,
        LrFlooded,

        CrTrapped,
        CrTreasure,
        CrGuarded,
        CrChaos,
        CrBasic,
        CrFlooded,

        HrTrapped,
        HrTreasure,
        HrGuarded,
        HrChaos,
        HrBasic,
        HrFlooded,

        ScTrapped,
        ScTreasure,
        ScGuarded,
        ScChaos,
        ScBasic,
        ScFlooded,
        
        LcTrapped,
        LcTreasure,
        LcHealthy,
        LcGuarded,
        LcChaos,
        LcBasic,
        LcFlooded,
    }

    impl From<u8> for Theme {
        fn from(val: u8) -> Self {
            match val {
                0  => Theme::Null,
                1  => Theme::Empty,
                2  => Theme::Entrance,
                3  => Theme::DeTrapped,
                4  => Theme::DeTreasure,
                5  => Theme::DeHealthy,
                6  => Theme::DeGuarded,
                7  => Theme::BrHoard,
                8  => Theme::BrWizard,
                9  => Theme::BrWeak,
                10 => Theme::BrStrong,
                11 => Theme::BrGuarded,
                12 => Theme::BrDouble,
                13 => Theme::SrTrapped,
                14 => Theme::SrTreasure,
                15 => Theme::SrGuarded,
                16 => Theme::SrChaos,
                17 => Theme::SrBasic,
                18 => Theme::SrFlooded,
                19 => Theme::CnTrapped,
                20 => Theme::CnGuarded,
                21 => Theme::CnBasic,
                22 => Theme::CnFlooded,
                23 => Theme::LrTrapped,
                24 => Theme::LrTreasure,
                25 => Theme::LrHealthy,
                26 => Theme::LrGuarded,
                27 => Theme::LrChaos,
                28 => Theme::LrBasic,
                29 => Theme::LrFlooded,
                30 => Theme::CrTrapped,
                31 => Theme::CrTreasure,
                32 => Theme::CrGuarded,
                33 => Theme::CrChaos,
                34 => Theme::CrBasic,
                35 => Theme::CrFlooded,
                36 => Theme::HrTrapped,
                37 => Theme::HrTreasure,
                38 => Theme::HrGuarded,
                39 => Theme::HrChaos,
                40 => Theme::HrBasic,
                41 => Theme::HrFlooded,
                42 => Theme::ScTrapped,
                43 => Theme::ScTreasure,
                44 => Theme::ScGuarded,
                45 => Theme::ScChaos,
                46 => Theme::ScBasic,
                47 => Theme::ScFlooded,
                48 => Theme::LcTrapped,
                49 => Theme::LcTreasure,
                50 => Theme::LcHealthy,
                51 => Theme::LcGuarded,
                52 => Theme::LcChaos,
                53 => Theme::LcBasic,
                54 => Theme::LcFlooded,
                _  => Theme::Null,
            }
        }
    }

    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Debug, Hash, Eq)]
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

    impl From<u8> for Tile {
        fn from(val: u8) -> Self {
            match val {
                0  => Tile::Wall,
                1  => Tile::Floor,
                2  => Tile::Hole,
                3  => Tile::Water,
                4  => Tile::WaterPool,
                5  => Tile::Trap,
                6  => Tile::HealingStation,
                7  => Tile::Chest,
                8  => Tile::LootPile,
                9  => Tile::LootCluster,
                10 => Tile::MonsterSpawner,
                11 => Tile::BossSpawner,
                12 => Tile::Shrine,
                13 => Tile::Entrance,
                14 => Tile::PaintRed,
                15 => Tile::PaintBlue,
                16 => Tile::PaintGreen,
                _  => Tile::Wall,
            }
        }
    }
}

pub mod feature_placement {
    use rand::{Rng, rngs::StdRng};

    use crate::helpers::enums::{Theme, Tile};

    #[derive(Clone, Copy)]
    enum Value {
        Single(u8),
        Range(u8, u8)
    }
    impl Value {
        fn resolve(self, rng: &mut StdRng) -> u8 {
            match self {
                Value::Single(x) => x,
                Value::Range(min, max) => rng.random_range(min..=max),
            }
        }
    }

    type Table = &'static [(Tile, Value)];

    fn table(theme: Theme) -> Table {
        match theme {
            Theme::DeTrapped => &[
                (Tile::Hole, Value::Single(1)),
                (Tile::Water, Value::Range(0, 1)),
                (Tile::Trap, Value::Single(3)),
            ],
            Theme::DeTreasure => &[
                (Tile::Trap, Value::Single(1)),
                (Tile::Chest, Value::Single(1)),
                (Tile::LootPile, Value::Single(2)),
                (Tile::MonsterSpawner, Value::Single(1)),
            ],
            Theme::DeHealthy => &[(Tile::HealingStation, Value::Single(1))],
            Theme::DeGuarded => &[(Tile::MonsterSpawner, Value::Single(1))],

            Theme::SrTrapped => &[
                (Tile::Hole, Value::Single(1)),
                (Tile::Trap, Value::Range(3, 5)),
                (Tile::LootPile, Value::Single(1)),
                (Tile::MonsterSpawner, Value::Single(1)),
            ],
            Theme::SrTreasure => &[
                (Tile::Trap, Value::Range(1, 2)),
                (Tile::Chest, Value::Single(2)),
                (Tile::LootPile, Value::Single(3)),
            ],
            Theme::SrGuarded => &[
                (Tile::Water, Value::Range(0, 1)),
                (Tile::Trap, Value::Single(1)),
                (Tile::LootPile, Value::Single(1)),
                (Tile::MonsterSpawner, Value::Single(2)),
            ],
            Theme::SrChaos => &[
                (Tile::Hole, Value::Single(2)),
                (Tile::Water, Value::Range(0, 1)),
                (Tile::Trap, Value::Single(3)),
                (Tile::Chest, Value::Single(1)),
                (Tile::LootPile, Value::Single(2)),
                (Tile::MonsterSpawner, Value::Single(3)),
                (Tile::Shrine, Value::Single(1)),
            ],
            Theme::SrBasic => &[
                (Tile::Trap, Value::Range(0, 1)),
                (Tile::LootPile, Value::Range(0, 1)),
            ],
            Theme::SrFlooded => &[
                (Tile::Water, Value::Single(5)),
                (Tile::WaterPool, Value::Single(13)),
                (Tile::MonsterSpawner, Value::Single(1)),
            ],

            Theme::CnTrapped => &[
                (Tile::Hole, Value::Single(1)),
                (Tile::Trap, Value::Range(1, 3)),
                (Tile::LootPile, Value::Single(1)),
            ],
            Theme::CnGuarded => &[(Tile::MonsterSpawner, Value::Single(1))],
            Theme::CnBasic => &[(Tile::LootPile, Value::Range(0, 1))],
            Theme::CnFlooded => &[
                (Tile::Water, Value::Single(4)),
                (Tile::WaterPool, Value::Single(10)),
                (Tile::MonsterSpawner, Value::Range(0, 1)),
            ],

            Theme::LrTrapped => &[
                (Tile::Hole, Value::Single(2)),
                (Tile::Water, Value::Single(1)),
                (Tile::Trap, Value::Range(3, 5)),
                (Tile::LootPile, Value::Single(2)),
                (Tile::MonsterSpawner, Value::Single(1)),
            ],
            Theme::LrTreasure => &[
                (Tile::Trap, Value::Single(1)),
                (Tile::Chest, Value::Single(2)),
                (Tile::LootPile, Value::Single(3)),
                (Tile::MonsterSpawner, Value::Single(1)),
            ],
            Theme::LrHealthy => &[(Tile::HealingStation, Value::Single(1))],
            Theme::LrGuarded => &[
                (Tile::Water, Value::Range(0, 1)),
                (Tile::Trap, Value::Single(1)),
                (Tile::Chest, Value::Single(1)),
                (Tile::LootPile, Value::Single(1)),
                (Tile::MonsterSpawner, Value::Single(3)),
            ],
            Theme::LrChaos => &[
                (Tile::Hole, Value::Single(2)),
                (Tile::Water, Value::Single(1)),
                (Tile::Trap, Value::Single(3)),
                (Tile::Chest, Value::Single(2)),
                (Tile::LootPile, Value::Single(3)),
                (Tile::MonsterSpawner, Value::Range(2, 4)),
                (Tile::Shrine, Value::Single(1)),
            ],
            Theme::LrBasic => &[
                (Tile::Trap, Value::Range(1, 2)),
                (Tile::LootPile, Value::Range(0, 1)),
            ],
            Theme::LrFlooded => &[
                (Tile::Water, Value::Single(6)),
                (Tile::WaterPool, Value::Single(18)),
                (Tile::MonsterSpawner, Value::Single(1)),
            ],

            Theme::CrTrapped => &[
                (Tile::Hole, Value::Single(1)),
                (Tile::Trap, Value::Range(2, 4)),
                (Tile::LootPile, Value::Single(1)),
            ],
            Theme::CrTreasure => &[
                (Tile::Trap, Value::Single(1)),
                (Tile::Chest, Value::Single(1)),
                (Tile::LootPile, Value::Single(3)),
                (Tile::MonsterSpawner, Value::Single(1)),
            ],
            Theme::CrGuarded => &[
                (Tile::Water, Value::Range(0, 1)),
                (Tile::Trap, Value::Single(1)),
                (Tile::LootPile, Value::Single(1)),
                (Tile::MonsterSpawner, Value::Single(2)),
            ],
            Theme::CrChaos => &[
                (Tile::Hole, Value::Range(0, 1)),
                (Tile::Water, Value::Single(1)),
                (Tile::Trap, Value::Single(3)),
                (Tile::Chest, Value::Range(0, 2)),
                (Tile::LootPile, Value::Single(3)),
                (Tile::MonsterSpawner, Value::Range(2, 3)),
                (Tile::Shrine, Value::Single(1)),
            ],
            Theme::CrBasic => &[
                (Tile::Trap, Value::Range(0, 1)),
                (Tile::LootPile, Value::Range(0, 1)),
            ],
            Theme::CrFlooded => &[
                (Tile::Water, Value::Single(4)),
                (Tile::WaterPool, Value::Single(12)),
                (Tile::MonsterSpawner, Value::Range(0, 1)),
            ],

            Theme::HrTrapped => &[
                (Tile::Hole, Value::Single(1)),
                (Tile::Trap, Value::Range(2, 4)),
                (Tile::LootPile, Value::Single(1)),
            ],
            Theme::HrTreasure => &[
                (Tile::Trap, Value::Single(1)),
                (Tile::Chest, Value::Single(1)),
                (Tile::LootPile, Value::Single(3)),
                (Tile::MonsterSpawner, Value::Single(1)),
            ],
            Theme::HrGuarded => &[
                (Tile::Water, Value::Range(0, 1)),
                (Tile::Trap, Value::Single(1)),
                (Tile::LootPile, Value::Single(1)),
                (Tile::MonsterSpawner, Value::Single(2)),
            ],
            Theme::HrChaos => &[
                (Tile::Hole, Value::Range(0, 1)),
                (Tile::Water, Value::Single(1)),
                (Tile::Trap, Value::Single(3)),
                (Tile::MonsterSpawner, Value::Range(2, 3)),
                (Tile::Shrine, Value::Single(1)),
                (Tile::Chest, Value::Range(0, 2)),
                (Tile::LootPile, Value::Single(3)),
            ],
            Theme::HrBasic => &[
                (Tile::Trap, Value::Range(0, 1)),
                (Tile::LootPile, Value::Range(0, 1)),
            ],
            Theme::HrFlooded => &[
                (Tile::Water, Value::Single(5)),
                (Tile::WaterPool, Value::Single(15)),
                (Tile::MonsterSpawner, Value::Range(0, 1)),
            ],

            Theme::BrHoard => &[
                (Tile::Chest, Value::Single(3)),
                (Tile::LootPile, Value::Single(9)),
                (Tile::BossSpawner, Value::Single(1)),
                (Tile::Shrine, Value::Single(1)),
            ],
            Theme::BrWizard => &[
                (Tile::Chest, Value::Single(4)),
                (Tile::LootPile, Value::Single(3)),
                (Tile::BossSpawner, Value::Single(1)),
                (Tile::Shrine, Value::Single(1)),
            ],
            Theme::BrWeak => &[
                (Tile::Trap, Value::Range(0, 1)),
                (Tile::Chest, Value::Single(1)),
                (Tile::LootPile, Value::Single(2)),
                (Tile::MonsterSpawner, Value::Single(1)),
                (Tile::BossSpawner, Value::Single(1)),
            ],
            Theme::BrStrong => &[
                (Tile::HealingStation, Value::Range(0, 1)),
                (Tile::Chest, Value::Range(1, 3)),
                (Tile::LootPile, Value::Single(5)),
                (Tile::BossSpawner, Value::Single(1)),
                (Tile::Shrine, Value::Single(1)),
            ],
            Theme::BrGuarded => &[
                (Tile::Trap, Value::Single(1)),
                (Tile::Chest, Value::Single(2)),
                (Tile::LootPile, Value::Single(3)),
                (Tile::MonsterSpawner, Value::Single(2)),
                (Tile::BossSpawner, Value::Single(1)),
                (Tile::Shrine, Value::Single(1)),
            ],
            Theme::BrDouble => &[
                (Tile::Chest, Value::Single(3)),
                (Tile::LootPile, Value::Single(5)),
                (Tile::BossSpawner, Value::Single(2)),
                (Tile::Shrine, Value::Single(1)),
            ],

            Theme::ScTrapped => &[
                (Tile::Hole, Value::Single(1)),
                (Tile::Trap, Value::Range(3, 5)),
                (Tile::LootPile, Value::Single(1)),
                (Tile::MonsterSpawner, Value::Single(1)),
            ],
            Theme::ScTreasure => &[
                (Tile::Trap, Value::Range(1, 2)),
                (Tile::Chest, Value::Single(2)),
                (Tile::LootPile, Value::Single(3)),
            ],
            Theme::ScGuarded => &[
                (Tile::Water, Value::Range(0, 1)),
                (Tile::Trap, Value::Single(1)),
                (Tile::LootPile, Value::Single(1)),
                (Tile::MonsterSpawner, Value::Single(2)),
            ],
            Theme::ScChaos => &[
                (Tile::Hole, Value::Single(2)),
                (Tile::Water, Value::Range(0, 1)),
                (Tile::Trap, Value::Single(3)),
                (Tile::Chest, Value::Single(1)),
                (Tile::LootPile, Value::Single(2)),
                (Tile::MonsterSpawner, Value::Single(3)),
                (Tile::Shrine, Value::Single(1)),
            ],
            Theme::ScBasic => &[
                (Tile::Trap, Value::Range(0, 1)),
                (Tile::LootPile, Value::Range(0, 1)),
            ],
            Theme::ScFlooded => &[
                (Tile::Water, Value::Single(4)),
                (Tile::WaterPool, Value::Single(12)),
                (Tile::MonsterSpawner, Value::Range(0, 1)),
            ],

            Theme::LcTrapped => &[
                (Tile::Hole, Value::Single(2)),
                (Tile::Water, Value::Single(1)),
                (Tile::Trap, Value::Range(3, 5)),
                (Tile::LootPile, Value::Single(2)),
                (Tile::MonsterSpawner, Value::Single(1)),
            ],
            Theme::LcTreasure => &[
                (Tile::Trap, Value::Single(1)),
                (Tile::Chest, Value::Single(2)),
                (Tile::LootPile, Value::Single(3)),
                (Tile::MonsterSpawner, Value::Single(1)),
            ],
            Theme::LcHealthy => &[(Tile::HealingStation, Value::Single(1))],
            Theme::LcGuarded => &[
                (Tile::Water, Value::Range(0, 1)),
                (Tile::Trap, Value::Single(1)),
                (Tile::Chest, Value::Single(1)),
                (Tile::LootPile, Value::Single(1)),
                (Tile::MonsterSpawner, Value::Single(3)),
            ],
            Theme::LcChaos => &[
                (Tile::Hole, Value::Single(2)),
                (Tile::Water, Value::Single(1)),
                (Tile::Trap, Value::Single(3)),
                (Tile::Chest, Value::Single(2)),
                (Tile::LootPile, Value::Single(3)),
                (Tile::MonsterSpawner, Value::Range(2, 4)),
                (Tile::Shrine, Value::Single(1)),
            ],
            Theme::LcBasic => &[
                (Tile::Trap, Value::Range(1, 2)),
                (Tile::LootPile, Value::Range(0, 1)),
            ],
            Theme::LcFlooded => &[
                (Tile::Water, Value::Single(7)),
                (Tile::WaterPool, Value::Single(21)),
                (Tile::MonsterSpawner, Value::Range(1, 2)),
            ],
            Theme::Empty | Theme::Null | Theme::Entrance => &[],
        }
    }

    pub fn map(theme: Theme, rng: &mut StdRng) -> [u8; 17] {
        let mut counts: [u8; 17] = [0u8; 17];
        for (tile, value) in table(theme) {
            counts[*tile as usize] = value.resolve(rng);
        }
        counts
    }

    pub struct ScanParams {
        pub require: u16,
        pub block: u16,
        pub bias: u16,
        pub place_on: Option<u16>
    }

    const NO_PARAMS: ScanParams = ScanParams{require: 0, block: 0, bias: 0, place_on: None};

    pub const SCAN_PARAMS: [ScanParams; 17] = {
        let mut p: [ScanParams; 17] = [NO_PARAMS; 17];

        const WALL:           u16 = 1 << 0;
        const FLOOR:          u16 = 1 << 1;
        const HOLE:           u16 = 1 << 2;
        const WATER:          u16 = 1 << 3;
        const WATER_POOL:     u16 = 1 << 4;
        const TRAP:           u16 = 1 << 5;
        const HEALING:        u16 = 1 << 6;
        const CHEST:          u16 = 1 << 7;
        const LOOT_PILE:      u16 = 1 << 8;
        const LOOT_CLUSTER:   u16 = 1 << 9;
        const MONSTER:        u16 = 1 << 10;
        const BOSS:           u16 = 1 << 11;
        const SHRINE:         u16 = 1 << 12;

        p[Tile::Entrance as usize]      = ScanParams { require: FLOOR, block: 0, bias: 0, place_on: Some(WALL) };
        p[Tile::Water as usize]         = ScanParams { require: 0, block: CHEST | LOOT_PILE | LOOT_CLUSTER | HOLE, bias: 0, place_on: None };
        p[Tile::WaterPool as usize]     = ScanParams { require: WATER, block: CHEST | LOOT_PILE | LOOT_CLUSTER | HOLE, bias: 0, place_on: None };
        p[Tile::Hole as usize]          = ScanParams { require: 0, block: WALL | WATER | WATER_POOL | LOOT_PILE | LOOT_CLUSTER, bias: 0, place_on: None };
        p[Tile::HealingStation as usize]= ScanParams { require: FLOOR, block: 0, bias: 0, place_on: Some(WALL) };
        p[Tile::Shrine as usize]        = ScanParams { require: FLOOR, block: 0, bias: 0, place_on: Some(WALL) };
        p[Tile::Chest as usize]         = ScanParams { require: 0, block: 0, bias: LOOT_PILE | LOOT_CLUSTER | WALL, place_on: None };
        p[Tile::LootPile as usize]      = ScanParams { require: 0, block: WATER | WATER_POOL | HOLE, bias: CHEST, place_on: None };
        p[Tile::LootCluster as usize]   = ScanParams { require: CHEST | LOOT_PILE | LOOT_CLUSTER, block: WATER | WATER_POOL | HOLE, bias: 0, place_on: None };
        p[Tile::Trap as usize]          = ScanParams { require: 0, block: TRAP | HEALING | SHRINE, bias: 0, place_on: None };
        p[Tile::BossSpawner as usize]   = ScanParams { require: 0, block: MONSTER | HEALING | SHRINE, bias: 0, place_on: None };
        p[Tile::MonsterSpawner as usize]= ScanParams { require: 0, block: BOSS | HEALING | SHRINE, bias: 0, place_on: None };
        p
    };
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
        [Theme::Null],
        [100]
    );
    pub const TABLE_ENTRANCE:     ([Theme; 1], [u8; 1]) = (
        [Theme::Entrance],
        [100]
    );
    pub const TABLE_DEAD_END:     ([Theme; 5], [u8; 5]) = (
        [Theme::DeTrapped, Theme::DeTreasure, Theme::DeHealthy, Theme::DeGuarded, Theme::Empty],
        [20, 15, 10, 15, 40]
    );
    pub const TABLE_BOSS_ROOM:    ([Theme; 6], [u8; 6]) = (
        [Theme::BrHoard, Theme::BrWizard, Theme::BrWeak, Theme::BrStrong, Theme::BrGuarded, Theme::BrDouble],
        [20, 20, 20, 10, 20, 10]
    );
    pub const TABLE_SMALL_ROOM:   ([Theme; 7], [u8; 7]) = (
        [Theme::SrTrapped, Theme::SrTreasure, Theme::SrGuarded, Theme::SrChaos, Theme::SrBasic, Theme::SrFlooded, Theme::Empty],
        [20, 10, 15, 10, 25, 10, 10]
    );
    pub const TABLE_CONNECTION:   ([Theme; 5], [u8; 5]) = (
        [Theme::CnTrapped, Theme::CnGuarded, Theme::CnBasic, Theme::CnFlooded, Theme::Empty],
        [20, 20, 25, 10, 25]
    );
    pub const TABLE_LARGE_ROOM:   ([Theme; 8], [u8; 8]) = (
        [Theme::LrTrapped, Theme::LrTreasure, Theme::LrHealthy, Theme::LrGuarded, Theme::LrChaos, Theme::LrBasic, Theme::LrFlooded, Theme::Empty],
        [20,  5,  5, 15, 10, 25, 10, 10]
    );
    pub const TABLE_CORNER:       ([Theme; 7], [u8; 7]) = (
        [Theme::CrTrapped, Theme::CrTreasure, Theme::CrGuarded, Theme::CrChaos, Theme::CrBasic, Theme::CrFlooded, Theme::Empty],
        [20, 10, 15, 10, 25, 10, 10]
    );
    pub const TABLE_HALF:         ([Theme; 7], [u8; 7]) = (
        [Theme::HrTrapped, Theme::HrTreasure, Theme::HrGuarded, Theme::HrChaos, Theme::HrBasic, Theme::HrFlooded, Theme::Empty],
        [20, 10, 15, 10, 25, 10, 10]
    );
    pub const TABLE_SMALL_CIRCLE: ([Theme; 7], [u8; 7]) = (
        [Theme::ScTrapped, Theme::ScTreasure, Theme::ScGuarded, Theme::ScChaos, Theme::ScBasic, Theme::ScFlooded, Theme::Empty],
        [20, 10, 15, 10, 25, 10, 10]
    );
    pub const TABLE_LARGE_CIRCLE: ([Theme; 8], [u8; 8]) = (
        [Theme::LcTrapped, Theme::LcTreasure, Theme::LcHealthy, Theme::LcGuarded, Theme::LcChaos, Theme::LcBasic, Theme::LcFlooded, Theme::Empty],
        [20,  5,  5, 15, 10, 25, 10, 10]
    );
}