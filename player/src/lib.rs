enum Item {
    Weapon,
    ArmourHead,
    ArmourChest,
    Consumable
}

struct Player {
    weapon_1: Option<Item::Weapon>,
    weapon_2: Option<Item>,
    armour_head: Option<Item>,
    armour_chest: Option<Item>,
    consumable_1: Option<Item>,
    consumable_2: Option<Item>,
    inventory: [Option<Item>; 8],
    money: u64,
    buffer: Option<Item>
}