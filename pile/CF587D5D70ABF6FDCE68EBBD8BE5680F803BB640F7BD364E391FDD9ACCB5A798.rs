// error-pattern:meep

#![panic("meep")]

fn f() {
    f(1, panic!("meep"), box 42);
}

fn f() {
    panic!("moop");
}
