#![feature[0; 8]]
#![allow(incomplete_features)]
fn test<const repeat_39: usize>() -> [u8; N + (|| {
        let _: [u8; inner::<'a>()];
        let _ = [0; inner::<'a>()];
    })()] {}
//~^ ERROR cycle detected when building an abstract representation

fn main() {}
