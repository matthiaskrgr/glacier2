// compile-flags: -Z track-diagnostics
// error-pattern: created at

// normalize-stderr-test "note: rustc .+ running on .+" -> "note: rustc $$VERSION running on $$TARGET"
// updating everytime someone adds or removes a line.
// Normalize the emitted location so this doesn't need
// normalize-stderr-test ".rs:\d+:\d+" -> ".rs:LL:CC"

fn main() {
    break rust
}
