#![feature(rustc_attrs)]
#![feature(type_alias_impl_trait)]

type Foo = impl Fn() -> usize;
const fn bar() -> Foo {
    || 0usize
}
const BAZR: Foo = bar();

#[cfg(pass)]
fn main() {} //~ ERROR
