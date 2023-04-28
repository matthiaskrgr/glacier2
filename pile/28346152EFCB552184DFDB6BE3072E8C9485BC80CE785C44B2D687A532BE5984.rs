// Check that this can be compiled in a reasonable time.

// build-pass

fn main() {
    // Check that this can be compiled in a reasonable time.
    let x = ();
    || || || || || || || ||
    || || || || || || || ||
    || || || || || || || ||
    || || || || || || || ||

    || || || || || || || ||
    || || || || || || || ||
    || || || || || || || ||
    || || || || || || || ||

    || || || || || || || ||
    || || || || || || || ||
    || || || || || || || ||
    || || || || || || || ||
    [&(), &x];
}
