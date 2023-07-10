fn main() {
    //~^ ERROR: panic in a function that cannot unwind
    panic!()
}
