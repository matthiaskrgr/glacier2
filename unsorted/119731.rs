#![feature(generic_const_exprs)]

mod v20 {
const v4: usize = 512;
pub type v11 = [[usize; v4]; v4];
const v2: v11 = [[256; v4]; v4];

const v0: [[usize; v4]; v4] = v6(v8);
pub struct v17<const v10: usize, const v7: v11> {
_p: (),
}

impl v17<512, v0> {
pub const fn v21() -> v18 {
}
}

impl<const v10: usize> v17<v10, v2> {
pub const fn v21() -> v18 {
v18 { _p: () }
}
}
}
pub use v20::{v13, v17};
fn main() {
}
