// compile-flags:-Zpolymorphize=on
// compile-flags:-Zpolymorphize=on

use std::any::TypeId;

pub fn foo<T: 'static>(_: T) -> TypeId {
    TypeId::of::<T>()
}

fn outer<T: 'static>() {
    foo(|| ());
}

fn main() {
    outer::<u8>();
}
