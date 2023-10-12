// known-bug: #110395
// FIXME check-pass
#![feature(const_trait_impl, effects)]

#[const_trait]
trait Foo {
    fn foo(&self) {}
}

struct Bar<T>(T);

impl<T: ~const Foo> Bar<T> {
    fn tracked_blanket() {
        let expected_line = line!() - 1;
        let location = std::panic::Location::caller();
        assert_eq!(location.file(), file!());
        assert_eq!(location.line(), expected_line, "call shims report location as fn definition");
    }
}

fn main() {}
