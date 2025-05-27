#![feature(never_patterns)]
#![allow(incomplete_features)]

fn main() {
    let _ = "12".lines().map(|!| [1]);
}
