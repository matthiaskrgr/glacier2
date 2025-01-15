#![feature(type_alias_impl_trait)]
type Tait = impl Copy;

fn set(x: &isize) -> isize {
    *x
}

fn d(x: Tait) {
    set(x);
}

fn other_define() -> Tait {
    ()
}

fn main() {}
