macro_rules! make_1 {
    () => {
        unused_braces
    };
}

fn make_1() {
    // ok
    let _: A<{ make_1!() }>; // ok
}
