#![feature(rustc_attrs)]
#![feature(type_alias_impl_trait)]

type Foo = impl Fn() -> usize;
const fn bar() -> Foo {
    || 43
}
const BAZR: Foo = bar();

#[rustc_error]
fn main() {} //~ ERROR
