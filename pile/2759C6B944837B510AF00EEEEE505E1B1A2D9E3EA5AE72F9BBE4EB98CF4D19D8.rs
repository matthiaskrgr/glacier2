// compile-flags:-Zpolymorphize=on
// compile-flags:-Zpolymorphize=on

fn test<T>() {
    std::mem::size_of::<T>();
}

pub fn foo<T>(_: T) -> &'static fn() {
    &(test::<T> as fn())
}

fn outer<T>() {
    foo(|| ());
}

fn main() {
    outer::<u8>();
}
