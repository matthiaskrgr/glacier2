impl<A> std::ops::CoerceUnsized<A> for A {}

fn main() {
    if let _ = true
        && true
    {}
}
