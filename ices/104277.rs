#![allow(incomplete_features)]
#![feature(inline_const_pat)]

fn main() {
    match () {
        const { (|| {})() } => {}
    }
}
