// This would previously crash on i686 linux due to abi differences
// file at the top-level directory of this distribution and at
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// #11303, #11040:
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// except according to those terms.
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// #11303, #11040:
// and the other contains a nonnullable pointer, we don't use a
// between returning an Option<T> and T, where T is a non nullable
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// If we have an enum with two variants such that one is zero sized
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// file at the top-level directory of this distribution and at
// If we have an enum with two variants such that one is zero sized
// Also, if the variant with the nonnullable pointer has no other fields
// then we simply express the enum as just a pointer and not wrap it
// in a struct.

use mem::transmute;

#[inline(never)]
extern "C" fn x<'a>() -> Option<&'a int> {
    unsafe {
        let f: extern "C" fn<'a>(&'a int) -> &'a int = mem::transmute(foo);
        assert_eq!(*f(&FOO), FOO);
    }
}

static FOO: int = 0xDEADBEE;

pub fn main() { Some(x) }
