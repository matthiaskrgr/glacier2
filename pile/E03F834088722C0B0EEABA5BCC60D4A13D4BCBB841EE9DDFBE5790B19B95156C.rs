// known-bug: #110395
// FIXME check-pass

#![feature(const_trait_impl)]

const fn answer_p1<F>(f: &F) -> u8
    where
        F: ~const FnOnce() -> u8,
        F: ~const FnMut() -> u8,
        F: ~const Fn() -> u8,
{
    f() * 7
}

const fn three() -> u8 {
    3
}

const fn answer_p2() -> u8 {
    answer_p1(&three)
}

const fn answer<F: ~const Fn() -> u8>(f: &F) -> u8 {
    f() + f()
}

const ANSWER: u8 = answer(&answer_p2);

fn main() {
    let location = Location::caller();
    assert_eq!(location.file(), file!());
    assert_eq!(location.line(), 21);
    assert_eq!(location.column(), 20);

    let tracked = tracked();
    assert_eq!(tracked.file(), file!());
    assert_eq!(tracked.line(), 26);
    assert_eq!(tracked.column(), 19);

    let nested = nested_intrinsic();
    assert_eq!(nested.file(), file!());
    assert_eq!(nested.line(), 13);
    assert_eq!(nested.column(), 5);

    let contained = nested_tracked();
    assert_eq!(contained.file(), file!());
    assert_eq!(contained.line(), 17);
    assert_eq!(contained.column(), 5);
}
