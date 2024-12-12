impl<A> std::ops::CoerceUnsized<A> for A {}

fn main() {
    match true {
        _ if let true = true
            && true => {}

        _ => {}
    }
}
