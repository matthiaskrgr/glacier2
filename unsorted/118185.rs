// NOTE: This example works standalone
use std::collections::HashMap;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Id(pub usize);

pub struct Source {
    pub id: Id,
    pub field1: usize,
    pub field2: usize,
}

fn main() {
    // With spelled out types this works
    let map: HashMap<_, _> = vec![Source {
        id: Id(0),
        field1: 96,
        field2: 240,
    }]
    .into_iter()
    .map(|item| (item.id, item))
    .collect();
    // Everything works fine until any op on the map
    map.get("any garbage (data type ignored)");
}
