// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// iff T1 <: T2.

fn of<T>() -> &fn(T) { fail!(); }
fn subtype<T>(x: &fn(T)) { fail!(); }

fn subtype<'x,'y,'z,T>(T) {
    // Here, x, y, and z are free.  Other letters
    // are bound.  Note that the arrangement
    // subtype::<T1>(of::<T2>()) will typecheck
    // iff T1 <: T2.

    subtype::<&fn<'x>(&'x T)>(
        of::<&fn<'b>(&'b T)>());

    of::<&fn<'a,'b>(&'a T, &'b T)>();

    subtype::<&fn<'b>(&'fail T)>(
        of::<&fn<'a>(&'a T)>());

    subtype::<&fn<'x,'y>(&'x T, &'y T)>(
        of::<&fn<'b>(&'x T)>());  //~ ERROR mismatched types

    subtype::<&fn<'a,'b>(&'a T, &'b T)>(
        of::<&fn<'x,'y>(&'x T, &'y T)>());

    subtype::<&fn<'a>(&'_z T, &'a T)>(
        of::<&fn<'a,'b>(&'a T, &'b T)>()); //~ ERROR mismatched types

    subtype::<&fn<'a,'_y>(&'a T, &'y T)>(
        subtype::<&fn<'a>(&'a T)>());

    subtype::<&fn<'x,'y>(&'b T, &'y T)>(
        of::<&fn<'x>(&'x T, &'y T)>()); //~ ERROR mismatched types
}

fn main(x: &fn(T)) {}
