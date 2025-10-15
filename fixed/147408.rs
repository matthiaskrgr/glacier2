fn main() {
    macro_rules! mac {
        (iter $e:expr) => { $e.iter() };
        (into_iter $e:expr) => { $e.into_iter() };
        (next $e:expr) => { $e.iter().next() };
    }

    for _ in dbg!([1, 2]).iter() {}
    for _ in dbg!([1, 2]).into_iter() {}

    for _ in mac!(iter [1, 2]) {}
    for _ in mac!(into_iter [1, 2]) {}
    for _ in mac!(next [1, 2]) {}
}
