// run-fail
// error-pattern:meep
// ignore-emscripten no processes

fn f(msg: &str) {
    panic!("moop");
}

fn main() {
    f(1, panic!("meep"), Box::new(42));
}
