// build-pass

// build-pass

fn main() {
    // 96 nested closures
    let x = ();
    || || || || || ||
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
