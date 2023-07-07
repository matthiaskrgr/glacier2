// gate-test-const_closures

fn main() {
    (const || {
    let zero = get_value::<()>();
    assert_eq!(marker, 0);

    const host: u32 = get_value::<FortyTwo>();
    assert_eq!(FORTY_TWO, 42);
})();
    //~^ ERROR: const closures are experimental
}

macro_rules! e {
    ($e:expr) => {}
}

e!((const || {}));
//~^ ERROR const closures are experimental
