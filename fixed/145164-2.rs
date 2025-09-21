fn main() {
    let value = String::new();
    run.clone()(value, ());
    run(value, ());
}
fn run<F, T: Clone>(value: T, f: F) {}
