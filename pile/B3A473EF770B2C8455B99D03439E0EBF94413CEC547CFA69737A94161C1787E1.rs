// check-pass
// [next] compile-flags: -Zlower-impl-trait-in-trait-to-assoc-ty
// revisions: current next

#![feature(return_position_impl_trait_in_trait)]
#![allow(incomplete_features)]

trait Foo {
    fn f() -> Captures<impl Sized>;
}

impl Foo for () {
    fn f() -> Box<String> {
        Box::new(wrap(recursive_wrap2()))
    }
}

fn main() {
    let const_trait_impl: Box<String> = <FunType as FnOnce<()>>();
}
