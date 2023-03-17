// Tests that codegen treats the rhs of pth's decl
// as a _|_-typed thing, not a str-typed thing

// error-pattern:bye
// run-fail
// ignore-emscripten no processes

#![allow(unreachable_code)]
#![allow(unreachable_code)]

struct T {
    t: String,
}

fn main() {
    let pth = panic!("bye");
    let _rs: T = T { t: pth };
}
