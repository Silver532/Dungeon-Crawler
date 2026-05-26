#![allow(unused)]

enum Item {
    Weapon(Weapon),
    ArmourHead(ArmourHead),
    ArmourChest(ArmourChest),
    ArmourGreaves(ArmourGreaves),
    Consumable(Consumable),
}

enum Material {
    Wood,
    Stone,
    Cloth,
    Rope,
    Gold,
    Silver,
    Electrum,
    Copper,
    Bronze,
    Iron,
    Steel,
    Glass,
    Cobalt,
    Ruby,
    Titanium,
    Emerald,
    Tungsten,
    Diamond,
    Mithril,
    Obsidian,
    Adamantite,
    Soulstone,
    Living,
    Bloomshard,
    Pure,
}

struct MaterialStats {
    damage: i16,
    speed: f32,
    reach: f32,
    traits: Vec<ItemTrait>
}

enum ItemTrait {
    CritChance(f32),
    CritDamage(f32),
    Knockback(f32),
    KbResist(f32),
    Defence(f32),
    StunChance(f32),
    StatusDamage(f32),
    StatusChance(f32),
    Weight(f32),
    Pierce(f32),
    MaxMana(f32),
    ManaCost(f32),
    ManaRecharge(f32),
    Cooldown(f32),
    Vamp(f32),
    Thirsting(f32),
    Vorpal(f32),
    TitanKiller(f32),
    Kronos(f32),
}

enum DamageType {
    Blunt,
    Sharp,
    Stab,
    Burn,
    Freeze,
    Shock,
    Light,
    Dark,
    True,
}

enum TopType {
    SwordBlade,
    DaggerBlade,
    AxeHead,
    SpearHead,
    HammerHead,
    MaceHead,
    Claw,
    Crystal,
    Orb,
}

struct TopPart {
    top_type: TopType,
    material: Material,
    level: u8,
    damage_type: DamageType,
    damage: u16,
}

enum MidType {
    HandGuard,
    NoGuard,
    Chain,
    Gauntlet,
    PoleGrip,
    WrappedGrip,
    RodGrip,
}

struct MidPart {
    mid_type: MidType,
    material: Material,
    level: u8,
    reach: f32,
}

enum EndType {
    Pommel,
    RuneCap,
    CounterWeight,
    LongPole,
    Hook,
    ButtSpike,
}

struct EndPart {
    end_type: EndType,
    material: Material,
    level: u8,
    speed: f32,
}

enum SpecialType {
    Gemstone,
    Embossment,
    Rune,
    Shard,
    Coating,
}

struct SpecialPart {
    special_type: SpecialType,
    material: Material,
    level: u8,
    traits: Vec<ItemTrait>
}

struct WeaponStats {
    damage: u16,
    speed: f32,
    reach: f32,
    traits: Vec<ItemTrait>
}

struct Weapon {
    top: TopPart,
    mid: MidPart,
    end: EndPart,
    special: Option<SpecialPart>,
    stats: WeaponStats,
    level: u8,
}

enum ArmourTrait {
    Reflect(f32),
    Absorb(f32),
    TimeWarp(f32),
}

struct ArmourStats {
    defence: i16,
    health: i16,
    movement: f32,
    traits: Vec<ArmourTrait>
}

struct ArmourHead {
    material: Material,
    level: u8,
    crit_chance: f32,
    stats: ArmourStats,
}

struct ArmourChest {
    material: Material,
    level: u8,
    kb_resist: f32,
    stats: ArmourStats,
}

struct ArmourGreaves {
    material: Material,
    level: u8,
    dodge: f32,
    stats: ArmourStats,
}

enum Effects {
    Recall,
    Heal,
}

struct Consumable {
    name: String,
    description: String,
    effect: Effects,
}

struct Player {
    weapon_1: Option<Weapon>,
    weapon_2: Option<Weapon>,
    armour_head: Option<ArmourHead>,
    armour_chest: Option<ArmourChest>,
    armour_greaves: Option<ArmourGreaves>,
    consumable_1: Option<Consumable>,
    consumable_2: Option<Consumable>,
    inventory: [Option<Item>; 8],
    money: u64,
}