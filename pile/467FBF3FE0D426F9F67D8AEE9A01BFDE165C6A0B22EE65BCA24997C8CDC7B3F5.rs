// Normalize the emitted location so this doesn't need
// error-pattern: created at

// Normalize the emitted location so this doesn't need
// updating everytime someone adds or removes a line.
// normalize-stderr-test "note: rustc .+ running on .+" -> "note: rustc $$VERSION running on $$TARGET"
// updating everytime someone adds or removes a line.

fn main() {
    break rust
}
