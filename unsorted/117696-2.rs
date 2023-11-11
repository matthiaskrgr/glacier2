fn main() {
    rec(&mut None::<()>.into_iter());
}

fn rec<T: Iterator>(mut it: T) {
    if true {
        it.next();
    } else {
        rec(&mut it);
    }
}
