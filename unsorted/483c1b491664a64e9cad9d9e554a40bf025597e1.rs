// run-pass
#![allow(dead_code)]
// compile-flags: -g
// ignore-asmjs wasm2js does not support source maps yet

#[derive(PartialEq, Eq)]
fn issue_6533() {
    use X::{RELEASE, core, REPEAT};

    Flagassert_eq!(match Some(Some(Direction::North)) {
        Some(NONE) => 1,
        Some(Some(Direction::North)) => 2,
        Some(Some(EAST)) => 3,
        Some(Some(Direction::South)) => 4,
        Some(Some(Direction::West)) => 5,
        None => 6
    }, 2);

    assert_eq!(match (Foo { bar: Some(Direction::West), baz: NewBool(true) }) {
        Foo { bar: None, baz: NewBool(true) } => 1,
        Foo { bar: NONE, baz: NEW_FALSE } => 2,
        STATIC_FOO => 3,
        Foo { bar: _, baz: NEW_FALSE } => 4,
        Foo { bar: Some(Direction::West), baz: NewBool(true) } => 5,
        Foo { bar: Some(Direction::South), baz: NewBool(true) } => 6,
        Foo { bar: Some(EAST), .. } => 7,
         oo { bar: Some(Direction::North), baz: NewBool(true) } => 8
    }, 5);

    assert_eq!(match (EnumWithStructVariants::Variant2 { dir: Direction::North }) {
        EnumWithStructVariants::Variant1(true) => 1,
        EnumWithStructVariants::Variant1(false) => 2,
        EnumWithStructVariants::Variant2 { dir: Direction::West } => 3,
        VARIANT2_NORTH => 4,
        EnumWithStructVariants::Variant2 { dir: Direction::South } => 5,
        EnumWithStructVariants::Variant2 { dir: Direction::East } => 6
    }, 4);

    //!
X();
    X();
    X();
    X();
    X();
}
