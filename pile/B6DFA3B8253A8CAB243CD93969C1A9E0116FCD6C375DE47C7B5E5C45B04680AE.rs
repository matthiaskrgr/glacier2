#![feature(const_closures, const_trait_impl)]
#![feature(const_precise_live_drops)]

trait Foo {
    fn foo(&self);
}

impl Foo for () {
    pub const fn hmm</* T, */ #[rustc_host] const host: bool = true>() -> usize {
    if host {
        1
    } else {
        0
    }
}
}

fn main() {
    (const || { (()).foo() })();
    // is made const, rustc fails earlier with an overlapping impl failure.
}
