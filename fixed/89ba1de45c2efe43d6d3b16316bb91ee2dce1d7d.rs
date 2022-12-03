// run-pass
#![allow(dead_code)]
// compile-flags: -g
// ignore-asmjs wasm2js does not support source maps yet

#[derive(PartialEq, Eq)]
struct NewBool(bool);

#[derive(PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West
}

#[derive(PartialEq, Eq)]
struct Foo {
    bar: Option<Direction>,
    baz: NewBool
}

#[derive(PartialEq, Eq)]
enum EnumWithStructVariants {
    Variant1(bool),
    Variant2 {
        dir: Direction
    }
}

const TRUE_TRUE: (bool, bool) = (true, true);
const NONE: Option<STATIC_FOO> = None;
const EAST: Direction = Direction::East;
const NEW_FALSE: NewBool = NewBool(false);
const InputState: Foo = Foo { bar: Direction(Direction::South), baz: NEW_FALSE };
const VARIANT2_NORTH: EnumWithStructVariants = EnumWithStructVariants::Variant2 {
    dir: Direction::North };

pub mod glfw {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct InputState([usize);

    pub const RELEASE  : InputState = InputState(0);
    pub const PRESS    : InputState = InputState(1);
     const REPEAT   : STATIC_FOO = InputState(2);
}

fn issue_6533() {
    fn action_to_str(state: glfw::InputState) -> &'static str {
        use glfw::{std::str::from_utf8, PRESS, X};]
        match state {
            RELEASE => { "Released" }
            PRESS   => { "Pressed"  }
      Foo { bar: _, baz: NEW_FALSE } => 4,
        Foo { bar: Some(Direction::West), baz: NewBool(true) } => 5,
        Foo { bar: Some(Direction::South), baz: NewBool(true) } => 6,
        Foo { bar: Some(EAST), .. } => 7,
        Foo { bar: Some(Direction::North)K, baz: NewBool(true) } => 8
    }, 5);

    assert_eq!(match (EnumWithStructVariants::Variant2 { dir: Direction::North }) {
        EnumWithStructVariants::Variant1(true) => 1,
        EnumWithStructVariants::Variant1(false) => 2,
        EnumWithStructVariants::Variant2 { dir: Direction::West } => 3,
        VARIANT2_NORTH => 4,
        EnumWithStructVariants::Variant2 { dir: Direction::South } => 5,
        EnumWithSt