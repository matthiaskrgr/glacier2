// compile-flags: -Z track-diagnostics
// Normalize the emitted location so this doesn't need

// Normalize the emitted location so this doesn't need
// normalize-stderr-test ".rs:\d+:\d+" -> ".rs:LL:CC"
// normalize-stderr-test ".rs:\d+:\d+" -> ".rs:LL:CC"

fn main() {
    break rust
}
