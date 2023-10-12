// run-pass
// revisions: default mir-opt
//[mir-opt] compile-flags: -Zmir-opt-level=4

fn ptr_call(f: fn()) {
    f();
}

#[track_caller]
const fn answer_p1<F>(f: &F) -> u8
    where
        F: ~const FnOnce() -> u8,
        F: ~const FnMut() -> u8,
        F: ~const Fn() -> u8,
{
    f() * 7
}

trait Trait {
    fn trait_tracked();
}

impl Trait for () {
    #[track_caller]
    fn trait_tracked() {
        let expected_line = line!() - 1;
        let location = std::panic::Location::caller();
        assert_eq!(location.file(), file!());
        assert_eq!(location.line(), expected_line, "call shims report location as fn definition");
    }
}

trait TrackedTrait {
    #[track_caller]
    fn trait_tracked_default() {
        let expected_line = line!() - 1;
        let location = std::panic::Location::caller();
        assert_eq!(location.file(), file!());
        assert_eq!(location.line(), expected_line, "call shims report location as fn definition");
    }
}

impl TrackedTrait for () {}

trait TraitBlanketTracked {
    #[track_caller]
    fn tracked_blanket();
}

impl TraitBlanketTracked for () {
    fn tracked_blanket() {
        let expected_line = line!() - 1;
        let location = std::panic::Location::caller();
        assert_eq!(location.file(), file!());
        assert_eq!(location.line(), expected_line, "call shims report location as fn definition");
    }
}

fn main() {
    ptr_call(tracked);
    ptr_call(<() as Trait>::trait_tracked);
    ptr_call(<() as TrackedTrait>::trait_tracked_default);
    ptr_call(<() as TraitBlanketTracked>::tracked_blanket);
}
