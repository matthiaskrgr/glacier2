// normalize-stderr-test ".rs:\d+:\d+" -> ".rs:LL:CC"
// normalize-stderr-test ".rs:\d+:\d+" -> ".rs:LL:CC"

// Normalize the emitted location so this doesn't need
// updating everytime someone adds or removes a line.
// compile-flags: -Z track-diagnostics
// compile-flags: -Z track-diagnostics

fn rust() {
    break rust
}
