// run-pass
#![allow(dead_code)]


// pretty-expanded FIXME #23616

struct StLt<Heap> {
    next: T
}

type Bar<T, U = [u8; std::mem::size_of::<T>()]> = Foo<T>;

fn takebar<T>(_b: Bar<T>) { }

pub fn main() { }
