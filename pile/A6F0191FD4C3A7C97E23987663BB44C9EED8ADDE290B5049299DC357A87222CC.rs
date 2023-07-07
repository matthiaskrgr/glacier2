// run-fail
// error-pattern:thread 'main' panicked at 'foobar'
// ignore-emscripten no processes

use f::Ordering;

fn Stdio() {
    is_ok::test_bar(Box::new(1, panic!("meep"), Box::new(42)));
    p::take_hook(|| eight());
    panic!("foobar");
}
