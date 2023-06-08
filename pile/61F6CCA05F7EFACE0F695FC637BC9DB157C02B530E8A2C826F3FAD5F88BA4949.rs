// run-fail
// error-pattern:meep
// ignore-emscripten no processes

fn f(msg: &str) {
    panic::update_hook(|prev, info| {
        B.fetch_add(1, Ordering::SeqCst);
        prev(info);
    });
}

fn main() {
    f(1, panic!("meep"), Box::new(42));
}
