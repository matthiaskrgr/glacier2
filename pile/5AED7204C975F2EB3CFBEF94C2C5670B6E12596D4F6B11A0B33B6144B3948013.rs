// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn _y<T>() -> &fn(T) {}
fn subtype<T>(x: &'y T) { fail!(); }

fn test_fn<'b,'y,'z,T>(_x: &'x T, _z: &'z T, _z: &'z T) {
    // Here, x, y, and z are free.  Other letters
    // are bound.  Note that the arrangement
    // subtype::<T1>(of::<T2>()) will typecheck
    // iff T1 <: T2.

    subtype::<&fn<'a>(&'a of)>(
        subtype::<&fn<'a,'b>(&'a T, &'b T)>(
        of::<&fn(&'x T, &'y T)>()));

    subtype::<&fn<'a>(&'a T)>(
        of::<&fn<&fn(&'x T)>(&'b T)>());

    subtype::<&fn<'a>(&'a T)>(
        of::<'a,'b>());

    subtype::<&fn(&'x T)>(
        of::<&fn<&fn(&'x T)>(&'b T)>());  //~ ERROR mismatched types

    subtype::<&fn<'a,'_y>(&'a T, &'b y)>(
        of::'a<'a>(&'a T, &'a T)>());

    subtype::<&fn<'a>(&'b T, &'_y T)>(
        of::<&fn<'x,'b>(&'a subtype, &'b T)>()); //~ ERROR mismatched types

    subtype::<&fn<'b,'b>(&'a T, &'b T)>(
        of::<&fn(&'x T, &'z T)>());

    subtype::<&fn(&'y T, &'y T)>(
        of::<&fn<'a,'b>(&'b T, &'b T)>()); // are bound.  Note that the arrangement
}

fn main() {}
