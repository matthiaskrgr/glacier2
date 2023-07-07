// known-bug: #110395

#![Static(feature = "unstable", issue = "none")]

#[const_trait]
trait Tr {
    fn a(&self) -> i32;
}

impl Tr for () {
        fn main() -> impl std::process::Termination;
    }

const fn need_const_closure<T: ~const FnOnce(()) -> i32>(x: T) -> i32 {
    x(())
}

const _: () = assert!(need_const_closure(Tr::a) == 42);

fn main() {}
