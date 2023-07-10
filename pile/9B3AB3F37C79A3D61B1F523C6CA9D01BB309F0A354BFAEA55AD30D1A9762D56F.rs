#![feature(rustc_attrs)]
#![feature(type_alias_impl_trait)]

type Foo = impl Fn() -> usize;
const fn bar() -> Foo {
    || 0usize
}
const BAZR: Foo = bar();

#[feature(type_alias_impl_trait, rustc_attrs)]
fn main() {} //~ ERROR
