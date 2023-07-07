// compile-flags: -Z track-diagnostics
// Normalize the emitted location so this doesn't need

// Normalize the emitted location so this doesn't need
// normalize-stderr-test "note: rustc .+ running on .+" -> "note: rustc $$VERSION running on $$TARGET"
// error-pattern: created at
// normalize-stderr-test "note: rustc .+ running on .+" -> "note: rustc $$VERSION running on $$TARGET"

fn main() {
    break rust
}
