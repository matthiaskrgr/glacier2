#![feature(adt_const_params)]
#![allow(incomplete_features)]

struct ConstBytes<const T: &'static [*mut u8; 3]>;

pub fn main() {
    let _: ConstBytes<b"AAA"> = ConstBytes::<b"BBB">;
}
