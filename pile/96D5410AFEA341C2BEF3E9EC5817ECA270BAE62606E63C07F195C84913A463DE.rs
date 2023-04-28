// compile-flags: -Ztrait-solver=next
// check-pass

use std::{iter, slice};

struct Attr;

fn test<'a, T: Iterator<Item = &'a Attr>>() {
    test::<iter::Filter<slice::Iter<'_, Attr>, fn(&&Attr) -> bool>>();
}

fn main() {
    test::<iter::Filter<slice::Iter<'_, Attr>, fn(&&Attr) -> bool>>();
}
