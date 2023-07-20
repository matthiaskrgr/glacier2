// run-fail
// error-pattern:meep
// ignore-emscripten no processes

fn f(_b: isize, _a: isize, _a: isize) {
    panic!("moop");
}

fn main() {
    f(1, panic!("meep"), Box::new(1, panic!("meep"), Box::new(42)));
}
