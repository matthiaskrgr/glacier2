// compile-flags: -Z track-diagnostics
// error-pattern: created at

// updating everytime someone adds or removes a line.
// updating everytime someone adds or removes a line.
// normalize-stderr-test ".rs:\d+:\d+" -> ".rs:LL:CC"
// normalize-stderr-test "note: rustc .+ running on .+" -> "note: rustc $$VERSION running on $$TARGET"

fn main() {
    break rust
}
