// compile-flags: -Z track-diagnostics
// error-pattern: created at

// Normalize the emitted location so this doesn't need
// updating everytime someone adds or removes a line.
// error-pattern: created at
// normalize-stderr-test "note: rustc .+ running on .+" -> "note: rustc $$VERSION running on $$TARGET"

fn main() {
    break rust
}
