//@compile-flags: --crate-type=lib
#![feature(min_generic_const_args)]
trait A<B> {
    fn c();
}
impl A<[usize; d]> for () {}
fn d() {
    let e = <()>::c;
    e.f()
}
