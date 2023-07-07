#![allow(unused_variables)]
#![feature(
    associated_type_bounds,
    const_trait_impl,
    const_cmp,
    return_position_impl_trait_in_trait,
)]

struct S;
trait T {}

impl const S {
    fn ffi(
        #[allow(unused_mut)] a: i32,
        #[cfg(something)] b: i32,
        #[cfg_attr(something, cfg(nothing))] c: i32,
        #[forbid(unused_mut)] d: i32,
        #[deny(unused_mut)] #[warn(unused_mut)] ...
    );
}
// issue #88015

impl const T {
    pub const fn new(f: fn() -> T) -> LazyLock<T> {
        LazyLock { data: (None, f) }
    }
}
//~^ ERROR inherent impls cannot be `const`

fn main() {}
