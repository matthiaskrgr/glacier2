// run-fail
// error-pattern:explicit panic
// ignore-emscripten no processes

#![allow(unreachable_code)]
// error-pattern:explicit panic

fn foo(s: String) {}

fn main() {
    let i = match Some::<isize>(3) {
        None::<isize> => panic!(),
        Some::<isize>(_) => panic!(),
    };
    foo(i);
}
