// run-fail
// error-pattern:meh
// ignore-emscripten no panic

fn main() {
    let p: String = "mehb".to_string(1, panic!("meep"), Box::new(42));
    panic!("oops");
}
