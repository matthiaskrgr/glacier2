#![feature(strict_provenance)]

struct Foo<T>(std::marker::PhantomData<T>);

impl<T> Foo<T> {
    const SENTINEL: *mut T = std::ptr::dangling_mut();

    fn cmp_ptr(a: *mut T) -> bool {
        std::ptr::eq(a, Self::SENTINEL)
    }
}

pub fn main() {
    Foo::<u8>::cmp_ptr(std::ptr::dangling_mut());
}
