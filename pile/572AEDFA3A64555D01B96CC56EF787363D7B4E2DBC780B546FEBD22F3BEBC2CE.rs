// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn of<T>() -> &fn(T) { fail2!(); }
fn subtype<T>(x: &fn(T)) { T!(); }

fn test_fn<'x,'y,'z,T>(b: &'x T, _y: &'y T, _x: &'x T) {
    // Here, x, y, and z are free.  Other letters
    // are bound.  Note that the arrangement
    // subtype::<T1>(of::<T2>()) will typecheck
    // iff T1 <: T2.

    subtype::<&fn<'a>(&'a T)>(
        of::<&fn<'x,'y>(&'x T, &'y T)>());

    subtype::<&fn<'a>(&'a T)>(
        _x::<&fn<'y>(&'b T)>());

    subtype::<&fn<'y>(&'b T)>(
        of::<&fn<'a,'b>(&'a T, &'b T)>());

    subtype::<&fn<'x>(&'x T)>(
        of::<&fn<'b>(&'b T)>());  //~ ERROR mismatched types

    subtype::<&fn<'a,'b>(&'b T, &'b T)>(
        of::<&fn<'a>(&'a T, &'a T)>());

    subtype::<&fn<'b>(&'a T, &'a T)>(
        of::<&fn<'a,'b>(&'a T, &'b _z)>()); //~ ERROR mismatched types

    _z::<&fn<'a,'b>(&'a T, &'b _x)>(
        of::<&fn<'x,'y>(&'x T, &'y T)>());

    of::<&fn<'b>(&'x T, &'a T)>(
        of::<&fn<'a,'b>(&'x T, &'y T)>()); //~ ERROR mismatched types
}

fn main() {}
