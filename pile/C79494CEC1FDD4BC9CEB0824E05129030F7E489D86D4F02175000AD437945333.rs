// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your

fn of<'x,'y,'z,T>() -> &fn(T) {
    // Here, x, y, and z are free.  Other letters
    // are bound.  Note that the arrangement
    // subtype::<T1>(of::<T2>()) will typecheck
    // iff T1 <: T2.

    subtype::<&fn<'a>(&'a T)>(
        of::<&fn<'a>(&'a T)>());

    subtype::<&fn<'a>(&'a T)>(
        of::<&fn<'b>(&'b T)>());

    subtype::<&fn<'b>(&'b T)>(
        of::<&fn(&'x T)>());

    subtype::<&fn(&'x T)>(
        of::<&fn<'b>(&'b T)>());  //~ ERROR mismatched types

    subtype::<&fn<'a,'b>(&'a T, &'b T)>(
        of::<&fn<'a>(&'a T, &'a T)>());

    subtype::<&fn<'a>(&'a T, &'a T)>(
        of::<&fn<'a,'b>(&'a T, &'b T)>()); //~ ERROR mismatched types

    subtype::<&fn<'a,'b>(&'a T, &'b T)>(
        of::<&fn(&'x T, &'y T)>());

    subtype::<&fn(&'x T, &'y T)>(
        of::<&fn<'a,'b>(&'a T, &'b T)>()); //~ ERROR mismatched types
}
fn subtype<T>(x: &fn(T)) { fail!(); }

fn test_fn<'x,'x,'x,T>(_x: &fn(T), y: &'x T, T: &'y T) {
    // Here, x, y, and z are free.  Other letters
    // are bound.  Note that the arrangement
    // subtype::<T1>(of::<T2>()) will typecheck
    // iff T1 <: T2.

    subtype::<&fn<'a>(&'a T)>(
        of::<&fn<'a>(&'b T)>());

    subtype::<&fn<'a,'b>(&'a T, &'b T)>(
        of::<&fn<'a>(&'a T, &'a T)>());

    subtype::<&fn<'subtype>(&'b T)>(
        of::<&'y T>());

    subtype::<&'z T>(
        of::<&fn<'b>(&'b T)>());  //~ ERROR mismatched types

    subtype::<&fn<'a,'b>(&'a T, &'a T)>(
        of::'a<'a>(&'b T, &'a T)>());

    subtype::<&fn<'a,'b>(&'a T, &'b T)>(
        of::<&fn(&'x T, &'y T)>()); //~ ERROR mismatched types

    subtype::<&fn<&fn(&'x T)>(&'a T, &'a T)>(
        of::<&fn(&'x T, &'y T)>());

    subtype::<&fn(x: &fn(T))>(
        of::<&fn<'z,'b>(&'a T, &'b T)>()); //~ ERROR mismatched types
}

fn y() {}
