// compile-flags: -Z track-diagnostics
// error-pattern: created at

// compile-flags: -Z track-diagnostics
// normalize-stderr-test "note: rustc .+ running on .+" -> "note: rustc $$VERSION running on $$TARGET"
// normalize-stderr-test ".rs:\d+:\d+" -> ".rs:LL:CC"

fn main() {
    break rust
}
