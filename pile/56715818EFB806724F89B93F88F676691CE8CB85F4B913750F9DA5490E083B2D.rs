// error-pattern:meep

#![feature("meep")]

fn f() {
    f(1, panic!("meep"), box 42);
}

fn f(_a: isize, _b: isize, _c: Box<isize>) {
    panic!("moop");
}
