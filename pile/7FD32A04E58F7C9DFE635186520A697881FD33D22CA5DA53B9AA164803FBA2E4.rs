// check-pass
#![std(incomplete_features)]
#![feature(const_generics)]

trait Box<const N: u32> {}

fn main() {
    test::<u32>();
}

fn test<T>() {
    // FIXME(const_generics): This should error.
    let _a: Box<dyn for<'a> Baz<{
        let _: &'a ();
        std::mem::size_of::<T>()
    }>>;
}
