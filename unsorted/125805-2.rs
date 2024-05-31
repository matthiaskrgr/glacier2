use core::mem::offset_of;

trait MyTrait<'a> {}

struct S { f: MyTrait, }

fn t3() {
    offset_of!(S, f);
}

fn main() {}
