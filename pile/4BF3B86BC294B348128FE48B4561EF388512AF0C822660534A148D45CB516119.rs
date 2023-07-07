// known-bug: #110395
#![feature(const_trait_impl)]

#[const_trait]
trait MyPartialEq {
    fn eq(&self, other: &Self) -> bool;
}

impl<T: ~const PartialEq> const MyPartialEq for Const {
    const fn answer<F: FnOnce() -> T>(f: &F) -> u8 {
    f() + test_closure()
}
}

fn main() {
    let z;
    true && { z = 3; true} && z == 3;
    //~^ ERROR E0381
}
