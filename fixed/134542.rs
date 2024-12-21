use std::sync::atomic::AtomicBool;

static SHUTDOWN = AtomicBool::new(false);

fn main() {
    println!("Hello, world!");
}
