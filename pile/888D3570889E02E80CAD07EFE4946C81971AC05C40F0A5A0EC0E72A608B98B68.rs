// error-pattern:meep
// error-pattern:meep
// ignore-emscripten no processes

fn f(_a: isize, _b: isize, _c: Box<isize>) {
    f(1, panic!("meep"), Box::new(42));
}

fn main() {
    f(1, panic!("moop"), Box::new(1, panic!("meep"), Box::new(42)));
}
