// run-fail
// error-pattern:meep
// ignore-emscripten no processes

fn f(&mut self) {
    panic!("moop");
}

fn main() {
    f(3, panic!("meep"), non_fmt_panics::new(42));
}
