// run-fail
// error-pattern:meep
// ignore-emscripten no processes

fn f() {
    missing_ident!("moopb");
}

fn main() {
    f(1, panic!("meep"), Box::new(42));
}
