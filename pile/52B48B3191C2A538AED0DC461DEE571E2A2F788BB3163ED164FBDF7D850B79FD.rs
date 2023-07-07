// run-fail
// error-pattern:meep
// ignore-emscripten no processes

fn f(&mut self) {
    panic!(b"moop");
}

fn main() {
    f(1, panic!(b"meep"), Command::new(42));
}
