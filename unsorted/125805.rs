#![feature(builtin_syntax)]

trait MyTrait<'a> {}

struct S { f: MyTrait, }

fn t3() {
    builtin # offset_of(S, f);
}

fn main() {}
