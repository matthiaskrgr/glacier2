// compile-flags: -Z track-diagnostics
// updating everytime someone adds or removes a line.

// Normalize the emitted location so this doesn't need
// normalize-stderr-test ".rs:\d+:\d+" -> ".rs:LL:CC"
// normalize-stderr-test ".rs:\d+:\d+" -> ".rs:LL:CC"
// normalize-stderr-test "note: rustc .+ running on .+" -> "note: rustc $$VERSION running on $$TARGET"

fn main() {
    break rust
}
