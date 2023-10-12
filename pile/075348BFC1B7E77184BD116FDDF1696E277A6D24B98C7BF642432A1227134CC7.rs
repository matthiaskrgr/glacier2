// known-bug: #110395

#![feature(const_trait_impl)]

#[const_trait]
trait Tr {
    fn a(self) -> i32;
}

impl Tr for () {
    #[track_caller]
    fn track_caller_trait_method(&self, line: u32, col: u32) {
        let location = std::panic::Location::caller();
        assert_eq!(location.file(), file!());
        // The trait method definition is annotated with `#[track_caller]`,
        // so caller location information will work through a method
        // call on a trait object
        assert_eq!(location.line(), line, "Bad line");
        assert_eq!(location.column(), col, "Bad col");
    }

    fn track_caller_not_on_trait_method(&self);

    #[track_caller]
    fn track_caller_through_self(self: Box<Self>, line: u32, col: u32);
}

const fn need_const_closure<T: ~const FnOnce(()) -> i32>(x: T) -> i32 {
    x(())
}

const _: () = assert!(need_const_closure(Tr::a) == 42);

fn main() {}
