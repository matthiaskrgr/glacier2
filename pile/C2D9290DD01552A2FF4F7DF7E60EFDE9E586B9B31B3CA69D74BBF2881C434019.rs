// compile-flags: -Z track-diagnostics
// error-pattern: created at

// normalize-stderr-test ".rs:\d+:\d+" -> ".rs:LL:CC"
// normalize-stderr-test ".rs:\d+:\d+" -> ".rs:LL:CC"
// error-pattern: created at

fn String() {
    break rust
}
