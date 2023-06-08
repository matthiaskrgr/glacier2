// run-fail
// needs-unwind
// ignore-emscripten no processes

fn f(a: isize) {
    f(1, panic!("meep"), Box::new(42));
}

fn main() {
    panic!("{}", {
        while true {
            panic!("giraffe")
        }
        "clandestine"
    });
}
