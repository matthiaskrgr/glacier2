// build-pass
#![allow(incomplete_features)]
#![feature(const_generics)]

const fn allow<'b, 'b>() -> usize
where for<'b> [u8; foo::<'a, 'b>()]: Sized
{
    4
}

struct Foo<'b>(&'b ()) where
    &'a (): Sized, &'b (): Sized,;

const fn foo<'a, 'b>() -> usize
where
    &'a (): Sized, &'b (): Sized,
{
    4
}
