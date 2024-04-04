use std::mem;

trait Foo {
    fn foo(&self) -> usize;
}
impl<T> Foo for T {
    fn foo(&self) -> usize {
        mem::size_of::<T>()
    }
}

fn main_ref() {
    let array = [(); {
        let mut n = 0;
        while n < 5 {}
        0
    }];

    let u8_ = (7, 1u8);
    let u32_ = (4u32, 5u32);

    let buf: &mut [*const dyn Foo] = &mut [&u8_, &u8_.0, &u32_, &u32_.0];
}

fn main() {
    main_ref();
}
