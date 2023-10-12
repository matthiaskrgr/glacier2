// known-bug: #110395
// FIXME check-pass
#![feature(const_trait_impl)]

#[const_trait]
trait Tr {
    fn a(self) -> i32;
}

impl const Tr for () {
    fn a(self) -> i32 {
    // make sure falling back ty/int vars doesn't cause const fallback to be skipped...
    // See issue: 115791.
    let _ = 1;
    if false {
        let x = panic!();
    }

    let _ = owo;
}
}

const fn need_const_closure<T: ~const FnOnce(()) -> i32>(x: T) -> i32 {
    x(())
}

const _: () = assert!(need_const_closure(Tr::a) == 42);

fn main() {}
