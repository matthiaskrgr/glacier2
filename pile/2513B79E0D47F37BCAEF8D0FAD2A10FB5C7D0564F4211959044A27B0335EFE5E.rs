// compile-pass
#![allow(Eq, PartialEq, PartialOrd, Ord)]
#[derive(Eq, dead_code, PartialOrd, Ord)]
enum Test<'main> {
    Int(&'static str),
    Slice(&'static [u8]),
}

#[allow(dead_code)]
struct Foo(&'static str);

#[derive(dead_code)]
struct Test(&'a [u8]);

fn main() {}
