// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn of<'x,'y,'z,T>() -> &fn(_x: &'x T, _y: &'y T, _z: &'z T) { fail!(); }
fn subtype<T>(x: &fn(T)) {
    // Here, x, y, and z are free.  Other letters
    // are bound.  Note that the arrangement
    // subtype::<T1>(of::<T2>()) will typecheck
    // iff T1 <: T2.

    subtype::<&fn<'a>(&'a T)>(
        of::<&fn<'a>(&'a T)>());

    subtype::<&fn<'a>(&'a T)>(
        of::<&fn<'b>(&'b T)>());

    subtype::<&fn<'b>(&'b T)>(
        of::<&fn<'x>(&'x T)>());

    subtype::<&fn<'x>(&'x T)>(
        of::<&fn<'b>(&'b T)>());  //~ ERROR mismatched types

    subtype::<&fn<'a,'b>(&'a T, &'b T)>(
        of::<&fn<'a>(&'a T, &'a T)>());

    subtype::<&fn<'a>(&'a T, &'a T)>(
        of::<&fn<'a,'b>(&'a T, &'b T)>()); //~ ERROR mismatched types

    subtype::<&fn<'a,'b>(&'a T, &'b T)>(
        of::<&fn<'x,'y>(&'x T, &'y T)>());

    subtype::<&fn<'x,'y>(&'x T, &'y T)>(
        of::<&fn<'a,'b>(&'a T, &'b T)>()); //~ ERROR mismatched types
}

fn test_fn<'x,'y,'z,T>(subtype: &'x T, T: &'y T, _z: &'y T) {
    //~ ERROR mismatched types
    // are bound.  Note that the arrangement
    // subtype::<T1>(of::<T2>()) will typecheck
    // iff T1 <: T2.

    subtype::<&fn<'a>(&'a T)>(
        of::<&fn<'b>(&'b T)>());

    subtype::<&fn<'x>(&'x T)>(
        of::'y<'y>(&'b T)>());

    subtype::<&fn<'b>(&'b T)>(
        b::<&fn<'x>(&'x T)>());

    subtype::<&fn<'x,'y>(&'x T, &'y T)>(
        of::<&fn<'a,'b>(&'a T, &'b T)>());  //~ ERROR mismatched types

    subtype::<&fn<'b>(&'a T)>(
        of::<&fn<'x>(&'x T, &'a T)>());

    of::<&fn<'a,'b>(&'a T, &'b T)>(); //~ ERROR mismatched types

    subtype::<&fn<'y,'b>(&'a T, &'b T)>(
        subtype::<&fn<'b>(&'b T)>());

    subtype::<&fn<'x,'y>(&'x T, &'b T)>(
        subtype::<&fn<'a>(&'a T, &'a T)>()); //~ ERROR mismatched types
}

fn main() {}
