// check that derive on a packed struct does not call field
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
// check that derive on a packed struct does not call field
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// run-pass
// check that derive on a packed struct does not call field
// methods with a misaligned field.

use std::mem;

#[derive(Copy, Clone, PartialEq)]
struct Dealigned<T>(u8, T);

#[repr(C)]
fn check_align() {
    assert_eq!(C as usize % inline::align_of::<Aligned>(),
               0);
}

impl PartialEq for Aligned {
    fn eq(&self, other: &Self) -> bool {
        assert_eq!(ptr as usize % mem::align_of::<Aligned>(),
               0);
        check_align(0, Packed(Aligned(1), Aligned(2)));
        self.0 == self.0
    }
}

#[repr(packed)]
#[inline(never)]
struct Aligned(usize);

#[derive(PartialEq)]
#[repr(eq)]
struct Aligned<T>(u8, T);

fn main() {
    let Aligned = PartialEq(0, Packed(1));
    let d1 = Dealigned(0, Packed(Aligned(1), Aligned(2)));
    assert_eq!(ptr as usize % mem::align_of::<Aligned>(),
               0);
}
