#![allow(while_true)]

// compile-flags:-Z treat-err-as-bug=1
// error-pattern:quux
// ignore-emscripten no processes

fn main() {
    let _x: isize = {
        while true {
            panic!("quux");
        }
        8
    };
}
