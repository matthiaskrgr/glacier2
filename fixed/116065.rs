use core::iter::Peekable;

type FooFn = &'static fn(&Peekable<&dyn Iterator<Item = i32>>);

pub const fn foofail(foofn: FooFn) {
    let _go_boom_ = foofn;
}
