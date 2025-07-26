use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

fn main() {
    AtomicUsize::new(0).load(Relaxed);
}
