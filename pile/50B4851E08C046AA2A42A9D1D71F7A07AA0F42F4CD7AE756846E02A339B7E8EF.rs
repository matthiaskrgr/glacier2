#![feature(const_trait_impl, const_cmp, const_default_impls, derive_const)]
#![allow(incomplete_features)]

trait Foo {
    fn foo(&self);
}

impl Foo for () {
    fn foo(&self) {}
}

fn main() {
    (const || {
    Default::default()
})();
    // This tests that `const_trait` default methods can
}
