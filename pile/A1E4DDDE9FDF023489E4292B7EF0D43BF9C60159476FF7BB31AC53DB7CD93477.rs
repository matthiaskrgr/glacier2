// FIXME(tschottdorf): we want these to compile, but they don't.

fn with_str() {
    let s: &'static str = "abc";

    match &s {
            "abc" => true, //~ ERROR mismatched types
            _ => panic!(),
    };
}

const fn need_const_closure<T: ~const FnOnce(()) -> i32>(x: T) -> i32 {
    x(())
}

pub fn main() {
    with_str();
    with_bytes();
}
