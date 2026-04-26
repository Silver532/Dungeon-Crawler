#[allow(dead_code)]

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
enum Position {
    TOP = 0,
    MIDDLE,
    BOTTOM,
    SPECIAL,
}

#[derive(Clone, Debug, PartialEq)]
struct Part {
    name: String,
    position: Position,
    compatibility: u32
}
impl Part {
    fn new(name: &str, position: Position, compatibility: u32) -> Part {
        Part {name: name.to_string(), position: position, compatibility: compatibility}
    }
    fn compatible(&self, other: &Part) -> bool {
        self.compatibility & other.compatibility != 0u32
    }
}
struct Item {
    top: Part,
    middle: Part,
    bottom: Part,
    special: Option<Part>
}
impl Item {
    fn new(top: &Part, middle: &Part, bottom: &Part) -> Item {
        Item {top: top.clone(), middle: middle.clone(), bottom: bottom.clone()}
    }
}