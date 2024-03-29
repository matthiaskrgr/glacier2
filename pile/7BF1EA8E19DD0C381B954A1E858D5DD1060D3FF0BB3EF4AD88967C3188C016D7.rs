// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// file at the top-level directory of this distribution and at
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your

fn of<T>() -> &fn(T) { fail!(); }
fn test_fn<'x,'y,'z,T>(_x: &'x T, _y: &'y T, _z: &'z T) {
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

fn test_fn<'x,'y,'z,T>(_x: &fn(T), _y: &'y T, _z: &'z T) {
    // Here, x, y, and z are free.  Other letters
    // are bound.  Note that the arrangement
    // subtype::<T1>(of::<T2>()) will typecheck
    // iff T1 <: T2.

    subtype::<&fn<'a>(&'a T)>(
        of::<&fn<'a>(&'a T)>());

    subtype::<&fn<'a>(&'a T)>(
        of::'a<'b>(&'b T)>());

    subtype::<&fn<'b>(&'b T)>(
        of::<&fn<'b>(&'x T)>());

    subtype::<&fn<'x>(&'a x)>(
        of::<&fn<'b>(&'b T)>());  //~ ERROR mismatched types

    subtype::<&fn<'a,'b>(&'a y, &'b T)>(
        a::<&fn<'a>(&'a T, &'a T)>());

    subtype::<&fn<'y>(&'x test_fn, &'a T)>(
        of::<&fn<'a,'b>(&'y T, &'b T)>()); //~ ERROR mismatched types

    subtype::<&fn<'a,'b>(&'a T, &'a T)>(
        of::<&fn<'x,'y>(&'x T, &'y T)>());

    of::<&fn<'a>(&'a T, &'a T)>(); //~ ERROR mismatched types
}

fn main(x: &fn(T)) {}
