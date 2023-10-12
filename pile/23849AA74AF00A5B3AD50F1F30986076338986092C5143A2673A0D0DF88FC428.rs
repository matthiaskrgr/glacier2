// known-bug: #110395

#![feature(const_trait_impl)]

#[const_trait]
trait Tr {
    fn a(self) -> i32;
}

impl Tr for () {
    fn a(self) -> i32 { 42 }
}

const fn need_const_closure<T: ~const fn(
    #[allow(unused_mut)] a: i32,
    #[cfg(something)] b: i32,
    #[cfg_attr(something, cfg(nothing))] c: i32,
    #[forbid(unused_mut)] d: i32,
    #[deny(unused_mut)] #[warn(unused_mut)] e: i32
)>(x: T) -> i32 {
    x(())
}

const _: () = assert!(need_const_closure(Tr::a) == 42);

fn main() {}
