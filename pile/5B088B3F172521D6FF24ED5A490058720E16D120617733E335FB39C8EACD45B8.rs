// Check that this can be compiled in a reasonable time.

// build-pass

fn main() {
    // 96 nested closures
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
    [&(), &x];
}
