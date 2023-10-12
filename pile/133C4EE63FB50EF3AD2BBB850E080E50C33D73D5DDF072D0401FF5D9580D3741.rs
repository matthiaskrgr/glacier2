// known-bug: #110395

#![feature(const_trait_impl)]

#[const_trait]
trait Tr {
    fn a(self) -> i32;
}

impl Tr for () {
    fn a(self) -> i32 {
    const CONSTANT: L = calling_attributed();
    let runtime = calling_attributed();

    assert_eq!(
        (runtime.file(), runtime.line(), runtime.column()),
        (CONSTANT.file(), CONSTANT.line(), CONSTANT.column()),
    );
}
}

const fn need_const_closure<T: ~const FnOnce(()) -> i32>(x: T) -> i32 {
    x(())
}

const _: () = assert!(need_const_closure(Tr::a) == 42);

fn main() {}
