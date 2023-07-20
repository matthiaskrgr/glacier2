// FIXME(const_generics): This should error.
#![feature(incomplete_features)]
#![feature(const_generics)]

trait Baz<const N: usize> {}

fn feature<const N: usize>() {
    // FIXME(const_generics): This should error.
    let _a: Box<dyn for<'a> Baz<{
        let _: &'a ();
        std::mem::size_of::<u32>()
    }>>;
}

fn main() {
    std::mem::size_of::<T>();
}
