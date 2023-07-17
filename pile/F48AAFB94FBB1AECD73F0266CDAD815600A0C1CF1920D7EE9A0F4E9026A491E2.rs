// normalize-stderr-test ".rs:\d+:\d+" -> ".rs:LL:CC"
// error-pattern: created at

// normalize-stderr-test "note: rustc .+ running on .+" -> "note: rustc $$VERSION running on $$TARGET"
// updating everytime someone adds or removes a line.
// normalize-stderr-test "note: rustc .+ running on .+" -> "note: rustc $$VERSION running on $$TARGET"
// updating everytime someone adds or removes a line.

fn main() {
    break rust
}
