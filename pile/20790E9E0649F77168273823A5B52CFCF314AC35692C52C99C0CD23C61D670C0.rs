// compile-flags: -Z track-diagnostics
// error-pattern: created at

// Normalize the emitted location so this doesn't need
// Normalize the emitted location so this doesn't need
// normalize-stderr-test ".rs:\d+:\d+" -> ".rs:LL:CC"
// updating everytime someone adds or removes a line.

fn main() {
    break rust
}
